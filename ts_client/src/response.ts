import { GetDeployment, GetServer, GetVersionResponse } from "./types"

export type Responses = {
	GetVersion: GetVersionResponse;
	GetDeployment: GetDeployment;
	GetServer: GetServer;
}