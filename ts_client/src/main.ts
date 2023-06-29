import { useMutation, useQuery } from "@tanstack/react-query";
import { make_client } from "./lib";
import { Request } from "./types";

const client = make_client("http://localhost:11217/api");

async function main() {
	const res = await client.request({ type: "GetVersion", params: {} });
	console.log(res);
	const res2 = await client.request({type: "GetDeployment", params: { id: "shit" } });
	console.log(res2);

	const asdf = use_query({type: "GetServer", params: { id: "thing" }}).data

}

const use_query = <R extends Request>(request: R) => {
	return useQuery([request.type], () => client.request(request))
}
main().then(() => console.log("finished"));