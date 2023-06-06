import { GetBuildResponse, GetDeploymentResponse, GetServerReponse, GetVersionResponse } from "./types"

export type Responses = {
	GetVersion: GetVersionResponse;
	GetServer: GetServerReponse;
	GetBuild: GetBuildResponse;
} & DeploymentResponses;

export type DeploymentResponses = {
	GetDeployment: GetDeploymentResponse
}