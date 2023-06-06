use yerpc::rpc;

pub struct Rpc;

#[rpc(all_positional, ts_outdir = "typescript/generated", openrpc_outdir = "./")]
impl Rpc {
	
}