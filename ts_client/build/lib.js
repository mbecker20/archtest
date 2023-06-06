"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.make_client = void 0;
const axios_1 = __importDefault(require("axios"));
function make_client(base_url) {
    const request = async (request) => {
        return await (0, axios_1.default)({
            method: "post",
            url: base_url,
            data: request,
        }).then(({ data }) => data);
    };
    return {
        request,
    };
}
exports.make_client = make_client;
