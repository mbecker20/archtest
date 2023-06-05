import { make_client } from "./lib";

async function main() {
	const client = make_client("http://localhost:11217/api");
	const res = await client.request({ type: "GetVersion" });
	console.log(res);
}

main().then(() => console.log("finished"));