use std::collections::HashMap;
use std::sync::Mutex;
use tonic::{Request, Response, Status};
use crate::proto::servers_server::Servers;
use crate::proto::*;
use rand::prelude::*;

#[derive(Debug)]
pub struct ServersSvc {
    servers: Mutex<HashMap<u32, Server>>,
    auth: Mutex<HashMap<u32, String>>,
}

impl ServersSvc {
    pub fn new() -> Self {
        ServersSvc {
            servers: Mutex::new(HashMap::new()),
            auth: Mutex::new(HashMap::new()),
        }
    }
}

#[tonic::async_trait]
impl Servers for ServersSvc {
    async fn list_servers(&self, req: Request<ListServersReq>) -> Result<Response<ListServersRes>, Status> {
        Ok(Response::new(ListServersRes{
            servers: self.servers.lock().unwrap().values().cloned().collect(),
        }))
    }

    async fn get_server(&self, req: Request<GetServerReq>) -> Result<Response<GetServerRes>, Status> {
        let uid = req.into_inner().uid;
        match self.servers.lock().unwrap().get(&uid) {
            Some(server) => Ok(Response::new(GetServerRes{ server: Some(server.clone()) })),
            None         => Err(Status::not_found("no such game server")),
        }
    }

    async fn create_server(&self, req: Request<CreateServerReq>) -> Result<Response<CreateServerRes>, Status> {
        let token = match req.metadata().get("authorization") {
            Some(t) => Ok(t),
            _       => Err(Status::unauthenticated("auth token required")),
        }?;
        let token = match token.to_str() {
            Ok(t) => Ok(t),
            _     => Err(Status::invalid_argument("invalid auth token format")),
        }?.to_string();
        if token.len() != 32 { Err(Status::invalid_argument("invalid token length")) } else { Ok(()) }?;

        let uid: u32 = random();
        let mut server = req.into_inner().server.ok_or(Status::invalid_argument("missing server spec"))?;
        server.uid = uid;
        self.servers.lock().unwrap().insert(uid, server);
        self.auth.lock().unwrap().insert(uid, token);
        Ok(Response::new(CreateServerRes{ uid }))
    }

    async fn update_server(&self, req: Request<UpdateServerReq>) -> Result<Response<UpdateServerRes>, Status> {
        todo!()
    }

    async fn remove_server(&self, req: Request<RemoveServerReq>) -> Result<Response<RemoveServerRes>, Status> {
        let token = match req.metadata().get("authorization") {
            Some(t) => Ok(t),
            _       => Err(Status::unauthenticated("auth token required")),
        }?;
        let token = match token.to_str() {
            Ok(t) => Ok(t),
            _     => Err(Status::invalid_argument("invalid auth token format")),
        }?.to_string();
        if token.len() != 32 { Err(Status::invalid_argument("invalid token length")) } else { Ok(()) }?;

        let uid = req.into_inner().uid;
        if *self.auth.lock().unwrap().get(&uid).ok_or(
            Status::not_found("no such game server")
        )? != token {
            Err(Status::permission_denied("access denied"))
        } else { Ok(()) }?;

        self.servers.lock().unwrap().remove(&uid);

        Ok(Response::new(RemoveServerRes{ uid }))
    }
}

