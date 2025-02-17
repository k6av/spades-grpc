use std::collections::HashMap;
use tonic::{Request, Response, Status};
use crate::proto::maps_server::Maps;
use crate::proto::*;

#[derive(Debug)]
pub struct MapsSvc {
    maps: HashMap<u32, Map>
}

impl MapsSvc {
    pub fn new(maps: HashMap<u32, Map>) -> Self {
        MapsSvc { maps }
    }
}

#[tonic::async_trait]
impl Maps for MapsSvc {
    async fn list_maps(&self, _req: Request<ListMapsReq>) -> Result<Response<ListMapsRes>, Status> {
        Ok(Response::new(ListMapsRes{
            maps: self.maps.values().cloned().collect(),
        }))
    }

    async fn get_map(&self, req: Request<GetMapReq>) -> Result<Response<GetMapRes>, Status> {
        let uid = req.into_inner().uid;
        match self.maps.get(&uid) {
            Some(map) => Ok(Response::new(GetMapRes{ map: Some(map.clone()) })),
            None      => Err(Status::not_found("no such game map")),
        }
    }
}

