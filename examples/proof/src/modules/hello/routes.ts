import { route } from "@velqu/core";
import { s } from "@velqu/schema";

export const hello = route({
  id: "hello.get",
  method: "GET",
  path: "/hello/:name",
  params: s.object({
    name: s.string({ minLength: 1, maxLength: 60 }),
  }),
  response: { 200: s.object({ message: s.string() }) },
  handle: ({ params }) => ({ message: `Hello ${params.name}` }),
});

export default hello;
