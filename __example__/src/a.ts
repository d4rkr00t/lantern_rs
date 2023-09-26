import b from "./b";
import { d } from "./b";

import { e } from "./e";

export function a() {
  return "a" + b() + d() + e.hello;
}

// unused
export { c } from "./b";
