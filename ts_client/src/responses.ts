import { GetBuild, GetDeployment, GetServer, GetVersionResponse } from "./types"

export type Responses = {
	GetVersion: GetVersionResponse;
	GetServer: GetServer;
	GetBuild: GetBuild;
} & DeploymentResponses;

export type DeploymentResponses = {
	GetDeployment: GetDeployment
}