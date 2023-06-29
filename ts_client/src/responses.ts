import { GetBuildResponse, GetDeploymentResponse, GetServerResponse, GetVersionResponse, OtherResponse } from "./types"

export type Responses = {
  GetVersion: GetVersionResponse;
  GetServer: GetServerResponse;
  GetBuild: GetBuildResponse;
  GetDeployment: GetDeploymentResponse;
  Other: OtherResponse
};