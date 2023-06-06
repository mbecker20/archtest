import { GetVersion, Id } from "./types"

export type Responses = {
	GetVersion: GetVersion;
	GetDeployment: Id;
	GetServer: Id;
}