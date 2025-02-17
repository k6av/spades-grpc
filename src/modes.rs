use std::collections::HashMap;
use tonic::{Request, Response, Status};
use crate::proto::modes_server::Modes;
use crate::proto::*;

#[derive(Debug)]
pub struct ModesSvc {
    modes: HashMap<u32, Mode>
}

impl ModesSvc {
    pub fn new(modes: HashMap<u32, Mode>) -> Self {
        ModesSvc { modes }
    }
}

#[tonic::async_trait]
impl Modes for ModesSvc {
    async fn list_modes(&self, req: Request<ListModesReq>) -> Result<Response<ListModesRes>, Status> {
        Ok(Response::new(ListModesRes{
            modes: self.modes.values().cloned().collect(),
        }))
    }

    async fn get_mode(&self, req: Request<GetModeReq>) -> Result<Response<GetModeRes>, Status> {
        let uid = req.into_inner().uid;
        match self.modes.get(&uid) {
            Some(mode) => Ok(Response::new(GetModeRes{ mode: Some(mode.clone()) })),
            None       => Err(Status::not_found("no such game mode")),
        }
    }
}


