import { f } from "./d";

export default function b() {
  return "b";
}

export function c() {
  f();
}

// unused
export function e() {}

export { default as d } from "./d";
