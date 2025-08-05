// Interface layer - Presentation layer (HTTP, gRPC, etc.)
// This layer handles external communication and user interface concerns

pub mod controllers;
pub mod grpc;
mod helper;
pub mod middleware;
pub mod routes;
