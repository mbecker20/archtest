import { Request } from "./types";
import { Responses } from "./response";
import axios from "axios";

export function make_client(base_url: string) {
  const request = async <R extends Request>(
    request: R
  ): Promise<Responses[R["type"]]> => {
    return await axios({
      method: "post",
      url: base_url,
      data: request,
    }).then(({ data }) => data);
  };

  return {
    request,
  };
}
