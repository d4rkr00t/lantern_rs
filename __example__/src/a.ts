import b from "./b";

export function a() {
  return "a" + b();
}

// unused
export { c } from "./b";
