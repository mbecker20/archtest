"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const lib_1 = require("./lib");
async function main() {
    const client = (0, lib_1.make_client)("http://localhost:11217/api");
    const res = await client.request({ type: "GetVersion" });
    console.log(res);
    const res2 = await client.request({ type: "GetDeployment", params: { id: "shit" } });
    console.log(res2);
}
main().then(() => console.log("finished"));
