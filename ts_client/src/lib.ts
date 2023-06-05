import { Request } from "./types";
import axios from "axios";

export function make_client(base_url: string) {
  const request = async (request: Request) => {
    return await axios({
      method: "post",
      url: base_url,
      data: request,
    }).then(({ data }) => data);
  };

	return {
		request,
	}
}
