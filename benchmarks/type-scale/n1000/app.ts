import { defineApp, defineModule, route } from "@q/core";
import { s } from "@q/schema";

const r0 = route({
  id: "res0.get",
  method: "GET",
  path: "/res0/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r1 = route({
  id: "res1.get",
  method: "GET",
  path: "/res1/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r2 = route({
  id: "res2.get",
  method: "GET",
  path: "/res2/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r3 = route({
  id: "res3.get",
  method: "GET",
  path: "/res3/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r4 = route({
  id: "res4.get",
  method: "GET",
  path: "/res4/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r5 = route({
  id: "res5.get",
  method: "GET",
  path: "/res5/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r6 = route({
  id: "res6.get",
  method: "GET",
  path: "/res6/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r7 = route({
  id: "res7.get",
  method: "GET",
  path: "/res7/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r8 = route({
  id: "res8.get",
  method: "GET",
  path: "/res8/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r9 = route({
  id: "res9.get",
  method: "GET",
  path: "/res9/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r10 = route({
  id: "res10.get",
  method: "GET",
  path: "/res10/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r11 = route({
  id: "res11.get",
  method: "GET",
  path: "/res11/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r12 = route({
  id: "res12.get",
  method: "GET",
  path: "/res12/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r13 = route({
  id: "res13.get",
  method: "GET",
  path: "/res13/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r14 = route({
  id: "res14.get",
  method: "GET",
  path: "/res14/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r15 = route({
  id: "res15.get",
  method: "GET",
  path: "/res15/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r16 = route({
  id: "res16.get",
  method: "GET",
  path: "/res16/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r17 = route({
  id: "res17.get",
  method: "GET",
  path: "/res17/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r18 = route({
  id: "res18.get",
  method: "GET",
  path: "/res18/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r19 = route({
  id: "res19.get",
  method: "GET",
  path: "/res19/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r20 = route({
  id: "res20.get",
  method: "GET",
  path: "/res20/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r21 = route({
  id: "res21.get",
  method: "GET",
  path: "/res21/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r22 = route({
  id: "res22.get",
  method: "GET",
  path: "/res22/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r23 = route({
  id: "res23.get",
  method: "GET",
  path: "/res23/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r24 = route({
  id: "res24.get",
  method: "GET",
  path: "/res24/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r25 = route({
  id: "res25.get",
  method: "GET",
  path: "/res25/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r26 = route({
  id: "res26.get",
  method: "GET",
  path: "/res26/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r27 = route({
  id: "res27.get",
  method: "GET",
  path: "/res27/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r28 = route({
  id: "res28.get",
  method: "GET",
  path: "/res28/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r29 = route({
  id: "res29.get",
  method: "GET",
  path: "/res29/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r30 = route({
  id: "res30.get",
  method: "GET",
  path: "/res30/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r31 = route({
  id: "res31.get",
  method: "GET",
  path: "/res31/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r32 = route({
  id: "res32.get",
  method: "GET",
  path: "/res32/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r33 = route({
  id: "res33.get",
  method: "GET",
  path: "/res33/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r34 = route({
  id: "res34.get",
  method: "GET",
  path: "/res34/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r35 = route({
  id: "res35.get",
  method: "GET",
  path: "/res35/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r36 = route({
  id: "res36.get",
  method: "GET",
  path: "/res36/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r37 = route({
  id: "res37.get",
  method: "GET",
  path: "/res37/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r38 = route({
  id: "res38.get",
  method: "GET",
  path: "/res38/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r39 = route({
  id: "res39.get",
  method: "GET",
  path: "/res39/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r40 = route({
  id: "res40.get",
  method: "GET",
  path: "/res40/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r41 = route({
  id: "res41.get",
  method: "GET",
  path: "/res41/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r42 = route({
  id: "res42.get",
  method: "GET",
  path: "/res42/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r43 = route({
  id: "res43.get",
  method: "GET",
  path: "/res43/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r44 = route({
  id: "res44.get",
  method: "GET",
  path: "/res44/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r45 = route({
  id: "res45.get",
  method: "GET",
  path: "/res45/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r46 = route({
  id: "res46.get",
  method: "GET",
  path: "/res46/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r47 = route({
  id: "res47.get",
  method: "GET",
  path: "/res47/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r48 = route({
  id: "res48.get",
  method: "GET",
  path: "/res48/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r49 = route({
  id: "res49.get",
  method: "GET",
  path: "/res49/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r50 = route({
  id: "res50.get",
  method: "GET",
  path: "/res50/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r51 = route({
  id: "res51.get",
  method: "GET",
  path: "/res51/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r52 = route({
  id: "res52.get",
  method: "GET",
  path: "/res52/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r53 = route({
  id: "res53.get",
  method: "GET",
  path: "/res53/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r54 = route({
  id: "res54.get",
  method: "GET",
  path: "/res54/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r55 = route({
  id: "res55.get",
  method: "GET",
  path: "/res55/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r56 = route({
  id: "res56.get",
  method: "GET",
  path: "/res56/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r57 = route({
  id: "res57.get",
  method: "GET",
  path: "/res57/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r58 = route({
  id: "res58.get",
  method: "GET",
  path: "/res58/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r59 = route({
  id: "res59.get",
  method: "GET",
  path: "/res59/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r60 = route({
  id: "res60.get",
  method: "GET",
  path: "/res60/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r61 = route({
  id: "res61.get",
  method: "GET",
  path: "/res61/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r62 = route({
  id: "res62.get",
  method: "GET",
  path: "/res62/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r63 = route({
  id: "res63.get",
  method: "GET",
  path: "/res63/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r64 = route({
  id: "res64.get",
  method: "GET",
  path: "/res64/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r65 = route({
  id: "res65.get",
  method: "GET",
  path: "/res65/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r66 = route({
  id: "res66.get",
  method: "GET",
  path: "/res66/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r67 = route({
  id: "res67.get",
  method: "GET",
  path: "/res67/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r68 = route({
  id: "res68.get",
  method: "GET",
  path: "/res68/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r69 = route({
  id: "res69.get",
  method: "GET",
  path: "/res69/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r70 = route({
  id: "res70.get",
  method: "GET",
  path: "/res70/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r71 = route({
  id: "res71.get",
  method: "GET",
  path: "/res71/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r72 = route({
  id: "res72.get",
  method: "GET",
  path: "/res72/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r73 = route({
  id: "res73.get",
  method: "GET",
  path: "/res73/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r74 = route({
  id: "res74.get",
  method: "GET",
  path: "/res74/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r75 = route({
  id: "res75.get",
  method: "GET",
  path: "/res75/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r76 = route({
  id: "res76.get",
  method: "GET",
  path: "/res76/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r77 = route({
  id: "res77.get",
  method: "GET",
  path: "/res77/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r78 = route({
  id: "res78.get",
  method: "GET",
  path: "/res78/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r79 = route({
  id: "res79.get",
  method: "GET",
  path: "/res79/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r80 = route({
  id: "res80.get",
  method: "GET",
  path: "/res80/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r81 = route({
  id: "res81.get",
  method: "GET",
  path: "/res81/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r82 = route({
  id: "res82.get",
  method: "GET",
  path: "/res82/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r83 = route({
  id: "res83.get",
  method: "GET",
  path: "/res83/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r84 = route({
  id: "res84.get",
  method: "GET",
  path: "/res84/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r85 = route({
  id: "res85.get",
  method: "GET",
  path: "/res85/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r86 = route({
  id: "res86.get",
  method: "GET",
  path: "/res86/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r87 = route({
  id: "res87.get",
  method: "GET",
  path: "/res87/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r88 = route({
  id: "res88.get",
  method: "GET",
  path: "/res88/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r89 = route({
  id: "res89.get",
  method: "GET",
  path: "/res89/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r90 = route({
  id: "res90.get",
  method: "GET",
  path: "/res90/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r91 = route({
  id: "res91.get",
  method: "GET",
  path: "/res91/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r92 = route({
  id: "res92.get",
  method: "GET",
  path: "/res92/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r93 = route({
  id: "res93.get",
  method: "GET",
  path: "/res93/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r94 = route({
  id: "res94.get",
  method: "GET",
  path: "/res94/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r95 = route({
  id: "res95.get",
  method: "GET",
  path: "/res95/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r96 = route({
  id: "res96.get",
  method: "GET",
  path: "/res96/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r97 = route({
  id: "res97.get",
  method: "GET",
  path: "/res97/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r98 = route({
  id: "res98.get",
  method: "GET",
  path: "/res98/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r99 = route({
  id: "res99.get",
  method: "GET",
  path: "/res99/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r100 = route({
  id: "res100.get",
  method: "GET",
  path: "/res100/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r101 = route({
  id: "res101.get",
  method: "GET",
  path: "/res101/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r102 = route({
  id: "res102.get",
  method: "GET",
  path: "/res102/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r103 = route({
  id: "res103.get",
  method: "GET",
  path: "/res103/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r104 = route({
  id: "res104.get",
  method: "GET",
  path: "/res104/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r105 = route({
  id: "res105.get",
  method: "GET",
  path: "/res105/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r106 = route({
  id: "res106.get",
  method: "GET",
  path: "/res106/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r107 = route({
  id: "res107.get",
  method: "GET",
  path: "/res107/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r108 = route({
  id: "res108.get",
  method: "GET",
  path: "/res108/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r109 = route({
  id: "res109.get",
  method: "GET",
  path: "/res109/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r110 = route({
  id: "res110.get",
  method: "GET",
  path: "/res110/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r111 = route({
  id: "res111.get",
  method: "GET",
  path: "/res111/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r112 = route({
  id: "res112.get",
  method: "GET",
  path: "/res112/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r113 = route({
  id: "res113.get",
  method: "GET",
  path: "/res113/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r114 = route({
  id: "res114.get",
  method: "GET",
  path: "/res114/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r115 = route({
  id: "res115.get",
  method: "GET",
  path: "/res115/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r116 = route({
  id: "res116.get",
  method: "GET",
  path: "/res116/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r117 = route({
  id: "res117.get",
  method: "GET",
  path: "/res117/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r118 = route({
  id: "res118.get",
  method: "GET",
  path: "/res118/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r119 = route({
  id: "res119.get",
  method: "GET",
  path: "/res119/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r120 = route({
  id: "res120.get",
  method: "GET",
  path: "/res120/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r121 = route({
  id: "res121.get",
  method: "GET",
  path: "/res121/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r122 = route({
  id: "res122.get",
  method: "GET",
  path: "/res122/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r123 = route({
  id: "res123.get",
  method: "GET",
  path: "/res123/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r124 = route({
  id: "res124.get",
  method: "GET",
  path: "/res124/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r125 = route({
  id: "res125.get",
  method: "GET",
  path: "/res125/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r126 = route({
  id: "res126.get",
  method: "GET",
  path: "/res126/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r127 = route({
  id: "res127.get",
  method: "GET",
  path: "/res127/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r128 = route({
  id: "res128.get",
  method: "GET",
  path: "/res128/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r129 = route({
  id: "res129.get",
  method: "GET",
  path: "/res129/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r130 = route({
  id: "res130.get",
  method: "GET",
  path: "/res130/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r131 = route({
  id: "res131.get",
  method: "GET",
  path: "/res131/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r132 = route({
  id: "res132.get",
  method: "GET",
  path: "/res132/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r133 = route({
  id: "res133.get",
  method: "GET",
  path: "/res133/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r134 = route({
  id: "res134.get",
  method: "GET",
  path: "/res134/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r135 = route({
  id: "res135.get",
  method: "GET",
  path: "/res135/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r136 = route({
  id: "res136.get",
  method: "GET",
  path: "/res136/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r137 = route({
  id: "res137.get",
  method: "GET",
  path: "/res137/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r138 = route({
  id: "res138.get",
  method: "GET",
  path: "/res138/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r139 = route({
  id: "res139.get",
  method: "GET",
  path: "/res139/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r140 = route({
  id: "res140.get",
  method: "GET",
  path: "/res140/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r141 = route({
  id: "res141.get",
  method: "GET",
  path: "/res141/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r142 = route({
  id: "res142.get",
  method: "GET",
  path: "/res142/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r143 = route({
  id: "res143.get",
  method: "GET",
  path: "/res143/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r144 = route({
  id: "res144.get",
  method: "GET",
  path: "/res144/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r145 = route({
  id: "res145.get",
  method: "GET",
  path: "/res145/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r146 = route({
  id: "res146.get",
  method: "GET",
  path: "/res146/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r147 = route({
  id: "res147.get",
  method: "GET",
  path: "/res147/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r148 = route({
  id: "res148.get",
  method: "GET",
  path: "/res148/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r149 = route({
  id: "res149.get",
  method: "GET",
  path: "/res149/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r150 = route({
  id: "res150.get",
  method: "GET",
  path: "/res150/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r151 = route({
  id: "res151.get",
  method: "GET",
  path: "/res151/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r152 = route({
  id: "res152.get",
  method: "GET",
  path: "/res152/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r153 = route({
  id: "res153.get",
  method: "GET",
  path: "/res153/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r154 = route({
  id: "res154.get",
  method: "GET",
  path: "/res154/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r155 = route({
  id: "res155.get",
  method: "GET",
  path: "/res155/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r156 = route({
  id: "res156.get",
  method: "GET",
  path: "/res156/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r157 = route({
  id: "res157.get",
  method: "GET",
  path: "/res157/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r158 = route({
  id: "res158.get",
  method: "GET",
  path: "/res158/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r159 = route({
  id: "res159.get",
  method: "GET",
  path: "/res159/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r160 = route({
  id: "res160.get",
  method: "GET",
  path: "/res160/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r161 = route({
  id: "res161.get",
  method: "GET",
  path: "/res161/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r162 = route({
  id: "res162.get",
  method: "GET",
  path: "/res162/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r163 = route({
  id: "res163.get",
  method: "GET",
  path: "/res163/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r164 = route({
  id: "res164.get",
  method: "GET",
  path: "/res164/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r165 = route({
  id: "res165.get",
  method: "GET",
  path: "/res165/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r166 = route({
  id: "res166.get",
  method: "GET",
  path: "/res166/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r167 = route({
  id: "res167.get",
  method: "GET",
  path: "/res167/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r168 = route({
  id: "res168.get",
  method: "GET",
  path: "/res168/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r169 = route({
  id: "res169.get",
  method: "GET",
  path: "/res169/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r170 = route({
  id: "res170.get",
  method: "GET",
  path: "/res170/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r171 = route({
  id: "res171.get",
  method: "GET",
  path: "/res171/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r172 = route({
  id: "res172.get",
  method: "GET",
  path: "/res172/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r173 = route({
  id: "res173.get",
  method: "GET",
  path: "/res173/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r174 = route({
  id: "res174.get",
  method: "GET",
  path: "/res174/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r175 = route({
  id: "res175.get",
  method: "GET",
  path: "/res175/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r176 = route({
  id: "res176.get",
  method: "GET",
  path: "/res176/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r177 = route({
  id: "res177.get",
  method: "GET",
  path: "/res177/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r178 = route({
  id: "res178.get",
  method: "GET",
  path: "/res178/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r179 = route({
  id: "res179.get",
  method: "GET",
  path: "/res179/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r180 = route({
  id: "res180.get",
  method: "GET",
  path: "/res180/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r181 = route({
  id: "res181.get",
  method: "GET",
  path: "/res181/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r182 = route({
  id: "res182.get",
  method: "GET",
  path: "/res182/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r183 = route({
  id: "res183.get",
  method: "GET",
  path: "/res183/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r184 = route({
  id: "res184.get",
  method: "GET",
  path: "/res184/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r185 = route({
  id: "res185.get",
  method: "GET",
  path: "/res185/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r186 = route({
  id: "res186.get",
  method: "GET",
  path: "/res186/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r187 = route({
  id: "res187.get",
  method: "GET",
  path: "/res187/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r188 = route({
  id: "res188.get",
  method: "GET",
  path: "/res188/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r189 = route({
  id: "res189.get",
  method: "GET",
  path: "/res189/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r190 = route({
  id: "res190.get",
  method: "GET",
  path: "/res190/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r191 = route({
  id: "res191.get",
  method: "GET",
  path: "/res191/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r192 = route({
  id: "res192.get",
  method: "GET",
  path: "/res192/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r193 = route({
  id: "res193.get",
  method: "GET",
  path: "/res193/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r194 = route({
  id: "res194.get",
  method: "GET",
  path: "/res194/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r195 = route({
  id: "res195.get",
  method: "GET",
  path: "/res195/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r196 = route({
  id: "res196.get",
  method: "GET",
  path: "/res196/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r197 = route({
  id: "res197.get",
  method: "GET",
  path: "/res197/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r198 = route({
  id: "res198.get",
  method: "GET",
  path: "/res198/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r199 = route({
  id: "res199.get",
  method: "GET",
  path: "/res199/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r200 = route({
  id: "res200.get",
  method: "GET",
  path: "/res200/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r201 = route({
  id: "res201.get",
  method: "GET",
  path: "/res201/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r202 = route({
  id: "res202.get",
  method: "GET",
  path: "/res202/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r203 = route({
  id: "res203.get",
  method: "GET",
  path: "/res203/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r204 = route({
  id: "res204.get",
  method: "GET",
  path: "/res204/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r205 = route({
  id: "res205.get",
  method: "GET",
  path: "/res205/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r206 = route({
  id: "res206.get",
  method: "GET",
  path: "/res206/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r207 = route({
  id: "res207.get",
  method: "GET",
  path: "/res207/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r208 = route({
  id: "res208.get",
  method: "GET",
  path: "/res208/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r209 = route({
  id: "res209.get",
  method: "GET",
  path: "/res209/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r210 = route({
  id: "res210.get",
  method: "GET",
  path: "/res210/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r211 = route({
  id: "res211.get",
  method: "GET",
  path: "/res211/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r212 = route({
  id: "res212.get",
  method: "GET",
  path: "/res212/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r213 = route({
  id: "res213.get",
  method: "GET",
  path: "/res213/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r214 = route({
  id: "res214.get",
  method: "GET",
  path: "/res214/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r215 = route({
  id: "res215.get",
  method: "GET",
  path: "/res215/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r216 = route({
  id: "res216.get",
  method: "GET",
  path: "/res216/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r217 = route({
  id: "res217.get",
  method: "GET",
  path: "/res217/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r218 = route({
  id: "res218.get",
  method: "GET",
  path: "/res218/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r219 = route({
  id: "res219.get",
  method: "GET",
  path: "/res219/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r220 = route({
  id: "res220.get",
  method: "GET",
  path: "/res220/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r221 = route({
  id: "res221.get",
  method: "GET",
  path: "/res221/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r222 = route({
  id: "res222.get",
  method: "GET",
  path: "/res222/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r223 = route({
  id: "res223.get",
  method: "GET",
  path: "/res223/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r224 = route({
  id: "res224.get",
  method: "GET",
  path: "/res224/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r225 = route({
  id: "res225.get",
  method: "GET",
  path: "/res225/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r226 = route({
  id: "res226.get",
  method: "GET",
  path: "/res226/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r227 = route({
  id: "res227.get",
  method: "GET",
  path: "/res227/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r228 = route({
  id: "res228.get",
  method: "GET",
  path: "/res228/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r229 = route({
  id: "res229.get",
  method: "GET",
  path: "/res229/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r230 = route({
  id: "res230.get",
  method: "GET",
  path: "/res230/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r231 = route({
  id: "res231.get",
  method: "GET",
  path: "/res231/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r232 = route({
  id: "res232.get",
  method: "GET",
  path: "/res232/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r233 = route({
  id: "res233.get",
  method: "GET",
  path: "/res233/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r234 = route({
  id: "res234.get",
  method: "GET",
  path: "/res234/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r235 = route({
  id: "res235.get",
  method: "GET",
  path: "/res235/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r236 = route({
  id: "res236.get",
  method: "GET",
  path: "/res236/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r237 = route({
  id: "res237.get",
  method: "GET",
  path: "/res237/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r238 = route({
  id: "res238.get",
  method: "GET",
  path: "/res238/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r239 = route({
  id: "res239.get",
  method: "GET",
  path: "/res239/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r240 = route({
  id: "res240.get",
  method: "GET",
  path: "/res240/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r241 = route({
  id: "res241.get",
  method: "GET",
  path: "/res241/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r242 = route({
  id: "res242.get",
  method: "GET",
  path: "/res242/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r243 = route({
  id: "res243.get",
  method: "GET",
  path: "/res243/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r244 = route({
  id: "res244.get",
  method: "GET",
  path: "/res244/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r245 = route({
  id: "res245.get",
  method: "GET",
  path: "/res245/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r246 = route({
  id: "res246.get",
  method: "GET",
  path: "/res246/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r247 = route({
  id: "res247.get",
  method: "GET",
  path: "/res247/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r248 = route({
  id: "res248.get",
  method: "GET",
  path: "/res248/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r249 = route({
  id: "res249.get",
  method: "GET",
  path: "/res249/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r250 = route({
  id: "res250.get",
  method: "GET",
  path: "/res250/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r251 = route({
  id: "res251.get",
  method: "GET",
  path: "/res251/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r252 = route({
  id: "res252.get",
  method: "GET",
  path: "/res252/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r253 = route({
  id: "res253.get",
  method: "GET",
  path: "/res253/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r254 = route({
  id: "res254.get",
  method: "GET",
  path: "/res254/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r255 = route({
  id: "res255.get",
  method: "GET",
  path: "/res255/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r256 = route({
  id: "res256.get",
  method: "GET",
  path: "/res256/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r257 = route({
  id: "res257.get",
  method: "GET",
  path: "/res257/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r258 = route({
  id: "res258.get",
  method: "GET",
  path: "/res258/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r259 = route({
  id: "res259.get",
  method: "GET",
  path: "/res259/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r260 = route({
  id: "res260.get",
  method: "GET",
  path: "/res260/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r261 = route({
  id: "res261.get",
  method: "GET",
  path: "/res261/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r262 = route({
  id: "res262.get",
  method: "GET",
  path: "/res262/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r263 = route({
  id: "res263.get",
  method: "GET",
  path: "/res263/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r264 = route({
  id: "res264.get",
  method: "GET",
  path: "/res264/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r265 = route({
  id: "res265.get",
  method: "GET",
  path: "/res265/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r266 = route({
  id: "res266.get",
  method: "GET",
  path: "/res266/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r267 = route({
  id: "res267.get",
  method: "GET",
  path: "/res267/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r268 = route({
  id: "res268.get",
  method: "GET",
  path: "/res268/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r269 = route({
  id: "res269.get",
  method: "GET",
  path: "/res269/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r270 = route({
  id: "res270.get",
  method: "GET",
  path: "/res270/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r271 = route({
  id: "res271.get",
  method: "GET",
  path: "/res271/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r272 = route({
  id: "res272.get",
  method: "GET",
  path: "/res272/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r273 = route({
  id: "res273.get",
  method: "GET",
  path: "/res273/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r274 = route({
  id: "res274.get",
  method: "GET",
  path: "/res274/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r275 = route({
  id: "res275.get",
  method: "GET",
  path: "/res275/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r276 = route({
  id: "res276.get",
  method: "GET",
  path: "/res276/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r277 = route({
  id: "res277.get",
  method: "GET",
  path: "/res277/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r278 = route({
  id: "res278.get",
  method: "GET",
  path: "/res278/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r279 = route({
  id: "res279.get",
  method: "GET",
  path: "/res279/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r280 = route({
  id: "res280.get",
  method: "GET",
  path: "/res280/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r281 = route({
  id: "res281.get",
  method: "GET",
  path: "/res281/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r282 = route({
  id: "res282.get",
  method: "GET",
  path: "/res282/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r283 = route({
  id: "res283.get",
  method: "GET",
  path: "/res283/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r284 = route({
  id: "res284.get",
  method: "GET",
  path: "/res284/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r285 = route({
  id: "res285.get",
  method: "GET",
  path: "/res285/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r286 = route({
  id: "res286.get",
  method: "GET",
  path: "/res286/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r287 = route({
  id: "res287.get",
  method: "GET",
  path: "/res287/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r288 = route({
  id: "res288.get",
  method: "GET",
  path: "/res288/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r289 = route({
  id: "res289.get",
  method: "GET",
  path: "/res289/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r290 = route({
  id: "res290.get",
  method: "GET",
  path: "/res290/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r291 = route({
  id: "res291.get",
  method: "GET",
  path: "/res291/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r292 = route({
  id: "res292.get",
  method: "GET",
  path: "/res292/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r293 = route({
  id: "res293.get",
  method: "GET",
  path: "/res293/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r294 = route({
  id: "res294.get",
  method: "GET",
  path: "/res294/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r295 = route({
  id: "res295.get",
  method: "GET",
  path: "/res295/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r296 = route({
  id: "res296.get",
  method: "GET",
  path: "/res296/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r297 = route({
  id: "res297.get",
  method: "GET",
  path: "/res297/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r298 = route({
  id: "res298.get",
  method: "GET",
  path: "/res298/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r299 = route({
  id: "res299.get",
  method: "GET",
  path: "/res299/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r300 = route({
  id: "res300.get",
  method: "GET",
  path: "/res300/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r301 = route({
  id: "res301.get",
  method: "GET",
  path: "/res301/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r302 = route({
  id: "res302.get",
  method: "GET",
  path: "/res302/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r303 = route({
  id: "res303.get",
  method: "GET",
  path: "/res303/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r304 = route({
  id: "res304.get",
  method: "GET",
  path: "/res304/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r305 = route({
  id: "res305.get",
  method: "GET",
  path: "/res305/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r306 = route({
  id: "res306.get",
  method: "GET",
  path: "/res306/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r307 = route({
  id: "res307.get",
  method: "GET",
  path: "/res307/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r308 = route({
  id: "res308.get",
  method: "GET",
  path: "/res308/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r309 = route({
  id: "res309.get",
  method: "GET",
  path: "/res309/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r310 = route({
  id: "res310.get",
  method: "GET",
  path: "/res310/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r311 = route({
  id: "res311.get",
  method: "GET",
  path: "/res311/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r312 = route({
  id: "res312.get",
  method: "GET",
  path: "/res312/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r313 = route({
  id: "res313.get",
  method: "GET",
  path: "/res313/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r314 = route({
  id: "res314.get",
  method: "GET",
  path: "/res314/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r315 = route({
  id: "res315.get",
  method: "GET",
  path: "/res315/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r316 = route({
  id: "res316.get",
  method: "GET",
  path: "/res316/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r317 = route({
  id: "res317.get",
  method: "GET",
  path: "/res317/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r318 = route({
  id: "res318.get",
  method: "GET",
  path: "/res318/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r319 = route({
  id: "res319.get",
  method: "GET",
  path: "/res319/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r320 = route({
  id: "res320.get",
  method: "GET",
  path: "/res320/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r321 = route({
  id: "res321.get",
  method: "GET",
  path: "/res321/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r322 = route({
  id: "res322.get",
  method: "GET",
  path: "/res322/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r323 = route({
  id: "res323.get",
  method: "GET",
  path: "/res323/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r324 = route({
  id: "res324.get",
  method: "GET",
  path: "/res324/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r325 = route({
  id: "res325.get",
  method: "GET",
  path: "/res325/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r326 = route({
  id: "res326.get",
  method: "GET",
  path: "/res326/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r327 = route({
  id: "res327.get",
  method: "GET",
  path: "/res327/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r328 = route({
  id: "res328.get",
  method: "GET",
  path: "/res328/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r329 = route({
  id: "res329.get",
  method: "GET",
  path: "/res329/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r330 = route({
  id: "res330.get",
  method: "GET",
  path: "/res330/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r331 = route({
  id: "res331.get",
  method: "GET",
  path: "/res331/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r332 = route({
  id: "res332.get",
  method: "GET",
  path: "/res332/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r333 = route({
  id: "res333.get",
  method: "GET",
  path: "/res333/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r334 = route({
  id: "res334.get",
  method: "GET",
  path: "/res334/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r335 = route({
  id: "res335.get",
  method: "GET",
  path: "/res335/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r336 = route({
  id: "res336.get",
  method: "GET",
  path: "/res336/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r337 = route({
  id: "res337.get",
  method: "GET",
  path: "/res337/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r338 = route({
  id: "res338.get",
  method: "GET",
  path: "/res338/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r339 = route({
  id: "res339.get",
  method: "GET",
  path: "/res339/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r340 = route({
  id: "res340.get",
  method: "GET",
  path: "/res340/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r341 = route({
  id: "res341.get",
  method: "GET",
  path: "/res341/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r342 = route({
  id: "res342.get",
  method: "GET",
  path: "/res342/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r343 = route({
  id: "res343.get",
  method: "GET",
  path: "/res343/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r344 = route({
  id: "res344.get",
  method: "GET",
  path: "/res344/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r345 = route({
  id: "res345.get",
  method: "GET",
  path: "/res345/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r346 = route({
  id: "res346.get",
  method: "GET",
  path: "/res346/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r347 = route({
  id: "res347.get",
  method: "GET",
  path: "/res347/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r348 = route({
  id: "res348.get",
  method: "GET",
  path: "/res348/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r349 = route({
  id: "res349.get",
  method: "GET",
  path: "/res349/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r350 = route({
  id: "res350.get",
  method: "GET",
  path: "/res350/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r351 = route({
  id: "res351.get",
  method: "GET",
  path: "/res351/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r352 = route({
  id: "res352.get",
  method: "GET",
  path: "/res352/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r353 = route({
  id: "res353.get",
  method: "GET",
  path: "/res353/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r354 = route({
  id: "res354.get",
  method: "GET",
  path: "/res354/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r355 = route({
  id: "res355.get",
  method: "GET",
  path: "/res355/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r356 = route({
  id: "res356.get",
  method: "GET",
  path: "/res356/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r357 = route({
  id: "res357.get",
  method: "GET",
  path: "/res357/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r358 = route({
  id: "res358.get",
  method: "GET",
  path: "/res358/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r359 = route({
  id: "res359.get",
  method: "GET",
  path: "/res359/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r360 = route({
  id: "res360.get",
  method: "GET",
  path: "/res360/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r361 = route({
  id: "res361.get",
  method: "GET",
  path: "/res361/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r362 = route({
  id: "res362.get",
  method: "GET",
  path: "/res362/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r363 = route({
  id: "res363.get",
  method: "GET",
  path: "/res363/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r364 = route({
  id: "res364.get",
  method: "GET",
  path: "/res364/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r365 = route({
  id: "res365.get",
  method: "GET",
  path: "/res365/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r366 = route({
  id: "res366.get",
  method: "GET",
  path: "/res366/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r367 = route({
  id: "res367.get",
  method: "GET",
  path: "/res367/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r368 = route({
  id: "res368.get",
  method: "GET",
  path: "/res368/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r369 = route({
  id: "res369.get",
  method: "GET",
  path: "/res369/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r370 = route({
  id: "res370.get",
  method: "GET",
  path: "/res370/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r371 = route({
  id: "res371.get",
  method: "GET",
  path: "/res371/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r372 = route({
  id: "res372.get",
  method: "GET",
  path: "/res372/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r373 = route({
  id: "res373.get",
  method: "GET",
  path: "/res373/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r374 = route({
  id: "res374.get",
  method: "GET",
  path: "/res374/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r375 = route({
  id: "res375.get",
  method: "GET",
  path: "/res375/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r376 = route({
  id: "res376.get",
  method: "GET",
  path: "/res376/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r377 = route({
  id: "res377.get",
  method: "GET",
  path: "/res377/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r378 = route({
  id: "res378.get",
  method: "GET",
  path: "/res378/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r379 = route({
  id: "res379.get",
  method: "GET",
  path: "/res379/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r380 = route({
  id: "res380.get",
  method: "GET",
  path: "/res380/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r381 = route({
  id: "res381.get",
  method: "GET",
  path: "/res381/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r382 = route({
  id: "res382.get",
  method: "GET",
  path: "/res382/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r383 = route({
  id: "res383.get",
  method: "GET",
  path: "/res383/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r384 = route({
  id: "res384.get",
  method: "GET",
  path: "/res384/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r385 = route({
  id: "res385.get",
  method: "GET",
  path: "/res385/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r386 = route({
  id: "res386.get",
  method: "GET",
  path: "/res386/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r387 = route({
  id: "res387.get",
  method: "GET",
  path: "/res387/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r388 = route({
  id: "res388.get",
  method: "GET",
  path: "/res388/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r389 = route({
  id: "res389.get",
  method: "GET",
  path: "/res389/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r390 = route({
  id: "res390.get",
  method: "GET",
  path: "/res390/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r391 = route({
  id: "res391.get",
  method: "GET",
  path: "/res391/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r392 = route({
  id: "res392.get",
  method: "GET",
  path: "/res392/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r393 = route({
  id: "res393.get",
  method: "GET",
  path: "/res393/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r394 = route({
  id: "res394.get",
  method: "GET",
  path: "/res394/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r395 = route({
  id: "res395.get",
  method: "GET",
  path: "/res395/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r396 = route({
  id: "res396.get",
  method: "GET",
  path: "/res396/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r397 = route({
  id: "res397.get",
  method: "GET",
  path: "/res397/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r398 = route({
  id: "res398.get",
  method: "GET",
  path: "/res398/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r399 = route({
  id: "res399.get",
  method: "GET",
  path: "/res399/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r400 = route({
  id: "res400.get",
  method: "GET",
  path: "/res400/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r401 = route({
  id: "res401.get",
  method: "GET",
  path: "/res401/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r402 = route({
  id: "res402.get",
  method: "GET",
  path: "/res402/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r403 = route({
  id: "res403.get",
  method: "GET",
  path: "/res403/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r404 = route({
  id: "res404.get",
  method: "GET",
  path: "/res404/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r405 = route({
  id: "res405.get",
  method: "GET",
  path: "/res405/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r406 = route({
  id: "res406.get",
  method: "GET",
  path: "/res406/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r407 = route({
  id: "res407.get",
  method: "GET",
  path: "/res407/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r408 = route({
  id: "res408.get",
  method: "GET",
  path: "/res408/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r409 = route({
  id: "res409.get",
  method: "GET",
  path: "/res409/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r410 = route({
  id: "res410.get",
  method: "GET",
  path: "/res410/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r411 = route({
  id: "res411.get",
  method: "GET",
  path: "/res411/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r412 = route({
  id: "res412.get",
  method: "GET",
  path: "/res412/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r413 = route({
  id: "res413.get",
  method: "GET",
  path: "/res413/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r414 = route({
  id: "res414.get",
  method: "GET",
  path: "/res414/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r415 = route({
  id: "res415.get",
  method: "GET",
  path: "/res415/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r416 = route({
  id: "res416.get",
  method: "GET",
  path: "/res416/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r417 = route({
  id: "res417.get",
  method: "GET",
  path: "/res417/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r418 = route({
  id: "res418.get",
  method: "GET",
  path: "/res418/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r419 = route({
  id: "res419.get",
  method: "GET",
  path: "/res419/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r420 = route({
  id: "res420.get",
  method: "GET",
  path: "/res420/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r421 = route({
  id: "res421.get",
  method: "GET",
  path: "/res421/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r422 = route({
  id: "res422.get",
  method: "GET",
  path: "/res422/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r423 = route({
  id: "res423.get",
  method: "GET",
  path: "/res423/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r424 = route({
  id: "res424.get",
  method: "GET",
  path: "/res424/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r425 = route({
  id: "res425.get",
  method: "GET",
  path: "/res425/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r426 = route({
  id: "res426.get",
  method: "GET",
  path: "/res426/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r427 = route({
  id: "res427.get",
  method: "GET",
  path: "/res427/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r428 = route({
  id: "res428.get",
  method: "GET",
  path: "/res428/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r429 = route({
  id: "res429.get",
  method: "GET",
  path: "/res429/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r430 = route({
  id: "res430.get",
  method: "GET",
  path: "/res430/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r431 = route({
  id: "res431.get",
  method: "GET",
  path: "/res431/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r432 = route({
  id: "res432.get",
  method: "GET",
  path: "/res432/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r433 = route({
  id: "res433.get",
  method: "GET",
  path: "/res433/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r434 = route({
  id: "res434.get",
  method: "GET",
  path: "/res434/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r435 = route({
  id: "res435.get",
  method: "GET",
  path: "/res435/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r436 = route({
  id: "res436.get",
  method: "GET",
  path: "/res436/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r437 = route({
  id: "res437.get",
  method: "GET",
  path: "/res437/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r438 = route({
  id: "res438.get",
  method: "GET",
  path: "/res438/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r439 = route({
  id: "res439.get",
  method: "GET",
  path: "/res439/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r440 = route({
  id: "res440.get",
  method: "GET",
  path: "/res440/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r441 = route({
  id: "res441.get",
  method: "GET",
  path: "/res441/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r442 = route({
  id: "res442.get",
  method: "GET",
  path: "/res442/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r443 = route({
  id: "res443.get",
  method: "GET",
  path: "/res443/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r444 = route({
  id: "res444.get",
  method: "GET",
  path: "/res444/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r445 = route({
  id: "res445.get",
  method: "GET",
  path: "/res445/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r446 = route({
  id: "res446.get",
  method: "GET",
  path: "/res446/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r447 = route({
  id: "res447.get",
  method: "GET",
  path: "/res447/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r448 = route({
  id: "res448.get",
  method: "GET",
  path: "/res448/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r449 = route({
  id: "res449.get",
  method: "GET",
  path: "/res449/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r450 = route({
  id: "res450.get",
  method: "GET",
  path: "/res450/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r451 = route({
  id: "res451.get",
  method: "GET",
  path: "/res451/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r452 = route({
  id: "res452.get",
  method: "GET",
  path: "/res452/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r453 = route({
  id: "res453.get",
  method: "GET",
  path: "/res453/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r454 = route({
  id: "res454.get",
  method: "GET",
  path: "/res454/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r455 = route({
  id: "res455.get",
  method: "GET",
  path: "/res455/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r456 = route({
  id: "res456.get",
  method: "GET",
  path: "/res456/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r457 = route({
  id: "res457.get",
  method: "GET",
  path: "/res457/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r458 = route({
  id: "res458.get",
  method: "GET",
  path: "/res458/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r459 = route({
  id: "res459.get",
  method: "GET",
  path: "/res459/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r460 = route({
  id: "res460.get",
  method: "GET",
  path: "/res460/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r461 = route({
  id: "res461.get",
  method: "GET",
  path: "/res461/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r462 = route({
  id: "res462.get",
  method: "GET",
  path: "/res462/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r463 = route({
  id: "res463.get",
  method: "GET",
  path: "/res463/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r464 = route({
  id: "res464.get",
  method: "GET",
  path: "/res464/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r465 = route({
  id: "res465.get",
  method: "GET",
  path: "/res465/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r466 = route({
  id: "res466.get",
  method: "GET",
  path: "/res466/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r467 = route({
  id: "res467.get",
  method: "GET",
  path: "/res467/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r468 = route({
  id: "res468.get",
  method: "GET",
  path: "/res468/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r469 = route({
  id: "res469.get",
  method: "GET",
  path: "/res469/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r470 = route({
  id: "res470.get",
  method: "GET",
  path: "/res470/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r471 = route({
  id: "res471.get",
  method: "GET",
  path: "/res471/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r472 = route({
  id: "res472.get",
  method: "GET",
  path: "/res472/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r473 = route({
  id: "res473.get",
  method: "GET",
  path: "/res473/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r474 = route({
  id: "res474.get",
  method: "GET",
  path: "/res474/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r475 = route({
  id: "res475.get",
  method: "GET",
  path: "/res475/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r476 = route({
  id: "res476.get",
  method: "GET",
  path: "/res476/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r477 = route({
  id: "res477.get",
  method: "GET",
  path: "/res477/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r478 = route({
  id: "res478.get",
  method: "GET",
  path: "/res478/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r479 = route({
  id: "res479.get",
  method: "GET",
  path: "/res479/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r480 = route({
  id: "res480.get",
  method: "GET",
  path: "/res480/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r481 = route({
  id: "res481.get",
  method: "GET",
  path: "/res481/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r482 = route({
  id: "res482.get",
  method: "GET",
  path: "/res482/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r483 = route({
  id: "res483.get",
  method: "GET",
  path: "/res483/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r484 = route({
  id: "res484.get",
  method: "GET",
  path: "/res484/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r485 = route({
  id: "res485.get",
  method: "GET",
  path: "/res485/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r486 = route({
  id: "res486.get",
  method: "GET",
  path: "/res486/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r487 = route({
  id: "res487.get",
  method: "GET",
  path: "/res487/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r488 = route({
  id: "res488.get",
  method: "GET",
  path: "/res488/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r489 = route({
  id: "res489.get",
  method: "GET",
  path: "/res489/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r490 = route({
  id: "res490.get",
  method: "GET",
  path: "/res490/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r491 = route({
  id: "res491.get",
  method: "GET",
  path: "/res491/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r492 = route({
  id: "res492.get",
  method: "GET",
  path: "/res492/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r493 = route({
  id: "res493.get",
  method: "GET",
  path: "/res493/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r494 = route({
  id: "res494.get",
  method: "GET",
  path: "/res494/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r495 = route({
  id: "res495.get",
  method: "GET",
  path: "/res495/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r496 = route({
  id: "res496.get",
  method: "GET",
  path: "/res496/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r497 = route({
  id: "res497.get",
  method: "GET",
  path: "/res497/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r498 = route({
  id: "res498.get",
  method: "GET",
  path: "/res498/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r499 = route({
  id: "res499.get",
  method: "GET",
  path: "/res499/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r500 = route({
  id: "res500.get",
  method: "GET",
  path: "/res500/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r501 = route({
  id: "res501.get",
  method: "GET",
  path: "/res501/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r502 = route({
  id: "res502.get",
  method: "GET",
  path: "/res502/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r503 = route({
  id: "res503.get",
  method: "GET",
  path: "/res503/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r504 = route({
  id: "res504.get",
  method: "GET",
  path: "/res504/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r505 = route({
  id: "res505.get",
  method: "GET",
  path: "/res505/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r506 = route({
  id: "res506.get",
  method: "GET",
  path: "/res506/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r507 = route({
  id: "res507.get",
  method: "GET",
  path: "/res507/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r508 = route({
  id: "res508.get",
  method: "GET",
  path: "/res508/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r509 = route({
  id: "res509.get",
  method: "GET",
  path: "/res509/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r510 = route({
  id: "res510.get",
  method: "GET",
  path: "/res510/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r511 = route({
  id: "res511.get",
  method: "GET",
  path: "/res511/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r512 = route({
  id: "res512.get",
  method: "GET",
  path: "/res512/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r513 = route({
  id: "res513.get",
  method: "GET",
  path: "/res513/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r514 = route({
  id: "res514.get",
  method: "GET",
  path: "/res514/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r515 = route({
  id: "res515.get",
  method: "GET",
  path: "/res515/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r516 = route({
  id: "res516.get",
  method: "GET",
  path: "/res516/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r517 = route({
  id: "res517.get",
  method: "GET",
  path: "/res517/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r518 = route({
  id: "res518.get",
  method: "GET",
  path: "/res518/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r519 = route({
  id: "res519.get",
  method: "GET",
  path: "/res519/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r520 = route({
  id: "res520.get",
  method: "GET",
  path: "/res520/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r521 = route({
  id: "res521.get",
  method: "GET",
  path: "/res521/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r522 = route({
  id: "res522.get",
  method: "GET",
  path: "/res522/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r523 = route({
  id: "res523.get",
  method: "GET",
  path: "/res523/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r524 = route({
  id: "res524.get",
  method: "GET",
  path: "/res524/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r525 = route({
  id: "res525.get",
  method: "GET",
  path: "/res525/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r526 = route({
  id: "res526.get",
  method: "GET",
  path: "/res526/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r527 = route({
  id: "res527.get",
  method: "GET",
  path: "/res527/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r528 = route({
  id: "res528.get",
  method: "GET",
  path: "/res528/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r529 = route({
  id: "res529.get",
  method: "GET",
  path: "/res529/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r530 = route({
  id: "res530.get",
  method: "GET",
  path: "/res530/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r531 = route({
  id: "res531.get",
  method: "GET",
  path: "/res531/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r532 = route({
  id: "res532.get",
  method: "GET",
  path: "/res532/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r533 = route({
  id: "res533.get",
  method: "GET",
  path: "/res533/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r534 = route({
  id: "res534.get",
  method: "GET",
  path: "/res534/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r535 = route({
  id: "res535.get",
  method: "GET",
  path: "/res535/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r536 = route({
  id: "res536.get",
  method: "GET",
  path: "/res536/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r537 = route({
  id: "res537.get",
  method: "GET",
  path: "/res537/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r538 = route({
  id: "res538.get",
  method: "GET",
  path: "/res538/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r539 = route({
  id: "res539.get",
  method: "GET",
  path: "/res539/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r540 = route({
  id: "res540.get",
  method: "GET",
  path: "/res540/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r541 = route({
  id: "res541.get",
  method: "GET",
  path: "/res541/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r542 = route({
  id: "res542.get",
  method: "GET",
  path: "/res542/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r543 = route({
  id: "res543.get",
  method: "GET",
  path: "/res543/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r544 = route({
  id: "res544.get",
  method: "GET",
  path: "/res544/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r545 = route({
  id: "res545.get",
  method: "GET",
  path: "/res545/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r546 = route({
  id: "res546.get",
  method: "GET",
  path: "/res546/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r547 = route({
  id: "res547.get",
  method: "GET",
  path: "/res547/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r548 = route({
  id: "res548.get",
  method: "GET",
  path: "/res548/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r549 = route({
  id: "res549.get",
  method: "GET",
  path: "/res549/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r550 = route({
  id: "res550.get",
  method: "GET",
  path: "/res550/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r551 = route({
  id: "res551.get",
  method: "GET",
  path: "/res551/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r552 = route({
  id: "res552.get",
  method: "GET",
  path: "/res552/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r553 = route({
  id: "res553.get",
  method: "GET",
  path: "/res553/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r554 = route({
  id: "res554.get",
  method: "GET",
  path: "/res554/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r555 = route({
  id: "res555.get",
  method: "GET",
  path: "/res555/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r556 = route({
  id: "res556.get",
  method: "GET",
  path: "/res556/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r557 = route({
  id: "res557.get",
  method: "GET",
  path: "/res557/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r558 = route({
  id: "res558.get",
  method: "GET",
  path: "/res558/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r559 = route({
  id: "res559.get",
  method: "GET",
  path: "/res559/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r560 = route({
  id: "res560.get",
  method: "GET",
  path: "/res560/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r561 = route({
  id: "res561.get",
  method: "GET",
  path: "/res561/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r562 = route({
  id: "res562.get",
  method: "GET",
  path: "/res562/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r563 = route({
  id: "res563.get",
  method: "GET",
  path: "/res563/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r564 = route({
  id: "res564.get",
  method: "GET",
  path: "/res564/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r565 = route({
  id: "res565.get",
  method: "GET",
  path: "/res565/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r566 = route({
  id: "res566.get",
  method: "GET",
  path: "/res566/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r567 = route({
  id: "res567.get",
  method: "GET",
  path: "/res567/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r568 = route({
  id: "res568.get",
  method: "GET",
  path: "/res568/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r569 = route({
  id: "res569.get",
  method: "GET",
  path: "/res569/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r570 = route({
  id: "res570.get",
  method: "GET",
  path: "/res570/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r571 = route({
  id: "res571.get",
  method: "GET",
  path: "/res571/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r572 = route({
  id: "res572.get",
  method: "GET",
  path: "/res572/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r573 = route({
  id: "res573.get",
  method: "GET",
  path: "/res573/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r574 = route({
  id: "res574.get",
  method: "GET",
  path: "/res574/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r575 = route({
  id: "res575.get",
  method: "GET",
  path: "/res575/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r576 = route({
  id: "res576.get",
  method: "GET",
  path: "/res576/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r577 = route({
  id: "res577.get",
  method: "GET",
  path: "/res577/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r578 = route({
  id: "res578.get",
  method: "GET",
  path: "/res578/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r579 = route({
  id: "res579.get",
  method: "GET",
  path: "/res579/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r580 = route({
  id: "res580.get",
  method: "GET",
  path: "/res580/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r581 = route({
  id: "res581.get",
  method: "GET",
  path: "/res581/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r582 = route({
  id: "res582.get",
  method: "GET",
  path: "/res582/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r583 = route({
  id: "res583.get",
  method: "GET",
  path: "/res583/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r584 = route({
  id: "res584.get",
  method: "GET",
  path: "/res584/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r585 = route({
  id: "res585.get",
  method: "GET",
  path: "/res585/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r586 = route({
  id: "res586.get",
  method: "GET",
  path: "/res586/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r587 = route({
  id: "res587.get",
  method: "GET",
  path: "/res587/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r588 = route({
  id: "res588.get",
  method: "GET",
  path: "/res588/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r589 = route({
  id: "res589.get",
  method: "GET",
  path: "/res589/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r590 = route({
  id: "res590.get",
  method: "GET",
  path: "/res590/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r591 = route({
  id: "res591.get",
  method: "GET",
  path: "/res591/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r592 = route({
  id: "res592.get",
  method: "GET",
  path: "/res592/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r593 = route({
  id: "res593.get",
  method: "GET",
  path: "/res593/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r594 = route({
  id: "res594.get",
  method: "GET",
  path: "/res594/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r595 = route({
  id: "res595.get",
  method: "GET",
  path: "/res595/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r596 = route({
  id: "res596.get",
  method: "GET",
  path: "/res596/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r597 = route({
  id: "res597.get",
  method: "GET",
  path: "/res597/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r598 = route({
  id: "res598.get",
  method: "GET",
  path: "/res598/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r599 = route({
  id: "res599.get",
  method: "GET",
  path: "/res599/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r600 = route({
  id: "res600.get",
  method: "GET",
  path: "/res600/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r601 = route({
  id: "res601.get",
  method: "GET",
  path: "/res601/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r602 = route({
  id: "res602.get",
  method: "GET",
  path: "/res602/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r603 = route({
  id: "res603.get",
  method: "GET",
  path: "/res603/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r604 = route({
  id: "res604.get",
  method: "GET",
  path: "/res604/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r605 = route({
  id: "res605.get",
  method: "GET",
  path: "/res605/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r606 = route({
  id: "res606.get",
  method: "GET",
  path: "/res606/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r607 = route({
  id: "res607.get",
  method: "GET",
  path: "/res607/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r608 = route({
  id: "res608.get",
  method: "GET",
  path: "/res608/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r609 = route({
  id: "res609.get",
  method: "GET",
  path: "/res609/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r610 = route({
  id: "res610.get",
  method: "GET",
  path: "/res610/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r611 = route({
  id: "res611.get",
  method: "GET",
  path: "/res611/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r612 = route({
  id: "res612.get",
  method: "GET",
  path: "/res612/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r613 = route({
  id: "res613.get",
  method: "GET",
  path: "/res613/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r614 = route({
  id: "res614.get",
  method: "GET",
  path: "/res614/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r615 = route({
  id: "res615.get",
  method: "GET",
  path: "/res615/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r616 = route({
  id: "res616.get",
  method: "GET",
  path: "/res616/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r617 = route({
  id: "res617.get",
  method: "GET",
  path: "/res617/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r618 = route({
  id: "res618.get",
  method: "GET",
  path: "/res618/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r619 = route({
  id: "res619.get",
  method: "GET",
  path: "/res619/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r620 = route({
  id: "res620.get",
  method: "GET",
  path: "/res620/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r621 = route({
  id: "res621.get",
  method: "GET",
  path: "/res621/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r622 = route({
  id: "res622.get",
  method: "GET",
  path: "/res622/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r623 = route({
  id: "res623.get",
  method: "GET",
  path: "/res623/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r624 = route({
  id: "res624.get",
  method: "GET",
  path: "/res624/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r625 = route({
  id: "res625.get",
  method: "GET",
  path: "/res625/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r626 = route({
  id: "res626.get",
  method: "GET",
  path: "/res626/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r627 = route({
  id: "res627.get",
  method: "GET",
  path: "/res627/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r628 = route({
  id: "res628.get",
  method: "GET",
  path: "/res628/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r629 = route({
  id: "res629.get",
  method: "GET",
  path: "/res629/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r630 = route({
  id: "res630.get",
  method: "GET",
  path: "/res630/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r631 = route({
  id: "res631.get",
  method: "GET",
  path: "/res631/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r632 = route({
  id: "res632.get",
  method: "GET",
  path: "/res632/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r633 = route({
  id: "res633.get",
  method: "GET",
  path: "/res633/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r634 = route({
  id: "res634.get",
  method: "GET",
  path: "/res634/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r635 = route({
  id: "res635.get",
  method: "GET",
  path: "/res635/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r636 = route({
  id: "res636.get",
  method: "GET",
  path: "/res636/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r637 = route({
  id: "res637.get",
  method: "GET",
  path: "/res637/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r638 = route({
  id: "res638.get",
  method: "GET",
  path: "/res638/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r639 = route({
  id: "res639.get",
  method: "GET",
  path: "/res639/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r640 = route({
  id: "res640.get",
  method: "GET",
  path: "/res640/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r641 = route({
  id: "res641.get",
  method: "GET",
  path: "/res641/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r642 = route({
  id: "res642.get",
  method: "GET",
  path: "/res642/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r643 = route({
  id: "res643.get",
  method: "GET",
  path: "/res643/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r644 = route({
  id: "res644.get",
  method: "GET",
  path: "/res644/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r645 = route({
  id: "res645.get",
  method: "GET",
  path: "/res645/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r646 = route({
  id: "res646.get",
  method: "GET",
  path: "/res646/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r647 = route({
  id: "res647.get",
  method: "GET",
  path: "/res647/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r648 = route({
  id: "res648.get",
  method: "GET",
  path: "/res648/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r649 = route({
  id: "res649.get",
  method: "GET",
  path: "/res649/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r650 = route({
  id: "res650.get",
  method: "GET",
  path: "/res650/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r651 = route({
  id: "res651.get",
  method: "GET",
  path: "/res651/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r652 = route({
  id: "res652.get",
  method: "GET",
  path: "/res652/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r653 = route({
  id: "res653.get",
  method: "GET",
  path: "/res653/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r654 = route({
  id: "res654.get",
  method: "GET",
  path: "/res654/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r655 = route({
  id: "res655.get",
  method: "GET",
  path: "/res655/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r656 = route({
  id: "res656.get",
  method: "GET",
  path: "/res656/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r657 = route({
  id: "res657.get",
  method: "GET",
  path: "/res657/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r658 = route({
  id: "res658.get",
  method: "GET",
  path: "/res658/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r659 = route({
  id: "res659.get",
  method: "GET",
  path: "/res659/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r660 = route({
  id: "res660.get",
  method: "GET",
  path: "/res660/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r661 = route({
  id: "res661.get",
  method: "GET",
  path: "/res661/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r662 = route({
  id: "res662.get",
  method: "GET",
  path: "/res662/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r663 = route({
  id: "res663.get",
  method: "GET",
  path: "/res663/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r664 = route({
  id: "res664.get",
  method: "GET",
  path: "/res664/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r665 = route({
  id: "res665.get",
  method: "GET",
  path: "/res665/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r666 = route({
  id: "res666.get",
  method: "GET",
  path: "/res666/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r667 = route({
  id: "res667.get",
  method: "GET",
  path: "/res667/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r668 = route({
  id: "res668.get",
  method: "GET",
  path: "/res668/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r669 = route({
  id: "res669.get",
  method: "GET",
  path: "/res669/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r670 = route({
  id: "res670.get",
  method: "GET",
  path: "/res670/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r671 = route({
  id: "res671.get",
  method: "GET",
  path: "/res671/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r672 = route({
  id: "res672.get",
  method: "GET",
  path: "/res672/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r673 = route({
  id: "res673.get",
  method: "GET",
  path: "/res673/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r674 = route({
  id: "res674.get",
  method: "GET",
  path: "/res674/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r675 = route({
  id: "res675.get",
  method: "GET",
  path: "/res675/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r676 = route({
  id: "res676.get",
  method: "GET",
  path: "/res676/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r677 = route({
  id: "res677.get",
  method: "GET",
  path: "/res677/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r678 = route({
  id: "res678.get",
  method: "GET",
  path: "/res678/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r679 = route({
  id: "res679.get",
  method: "GET",
  path: "/res679/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r680 = route({
  id: "res680.get",
  method: "GET",
  path: "/res680/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r681 = route({
  id: "res681.get",
  method: "GET",
  path: "/res681/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r682 = route({
  id: "res682.get",
  method: "GET",
  path: "/res682/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r683 = route({
  id: "res683.get",
  method: "GET",
  path: "/res683/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r684 = route({
  id: "res684.get",
  method: "GET",
  path: "/res684/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r685 = route({
  id: "res685.get",
  method: "GET",
  path: "/res685/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r686 = route({
  id: "res686.get",
  method: "GET",
  path: "/res686/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r687 = route({
  id: "res687.get",
  method: "GET",
  path: "/res687/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r688 = route({
  id: "res688.get",
  method: "GET",
  path: "/res688/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r689 = route({
  id: "res689.get",
  method: "GET",
  path: "/res689/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r690 = route({
  id: "res690.get",
  method: "GET",
  path: "/res690/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r691 = route({
  id: "res691.get",
  method: "GET",
  path: "/res691/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r692 = route({
  id: "res692.get",
  method: "GET",
  path: "/res692/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r693 = route({
  id: "res693.get",
  method: "GET",
  path: "/res693/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r694 = route({
  id: "res694.get",
  method: "GET",
  path: "/res694/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r695 = route({
  id: "res695.get",
  method: "GET",
  path: "/res695/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r696 = route({
  id: "res696.get",
  method: "GET",
  path: "/res696/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r697 = route({
  id: "res697.get",
  method: "GET",
  path: "/res697/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r698 = route({
  id: "res698.get",
  method: "GET",
  path: "/res698/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r699 = route({
  id: "res699.get",
  method: "GET",
  path: "/res699/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r700 = route({
  id: "res700.get",
  method: "GET",
  path: "/res700/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r701 = route({
  id: "res701.get",
  method: "GET",
  path: "/res701/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r702 = route({
  id: "res702.get",
  method: "GET",
  path: "/res702/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r703 = route({
  id: "res703.get",
  method: "GET",
  path: "/res703/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r704 = route({
  id: "res704.get",
  method: "GET",
  path: "/res704/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r705 = route({
  id: "res705.get",
  method: "GET",
  path: "/res705/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r706 = route({
  id: "res706.get",
  method: "GET",
  path: "/res706/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r707 = route({
  id: "res707.get",
  method: "GET",
  path: "/res707/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r708 = route({
  id: "res708.get",
  method: "GET",
  path: "/res708/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r709 = route({
  id: "res709.get",
  method: "GET",
  path: "/res709/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r710 = route({
  id: "res710.get",
  method: "GET",
  path: "/res710/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r711 = route({
  id: "res711.get",
  method: "GET",
  path: "/res711/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r712 = route({
  id: "res712.get",
  method: "GET",
  path: "/res712/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r713 = route({
  id: "res713.get",
  method: "GET",
  path: "/res713/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r714 = route({
  id: "res714.get",
  method: "GET",
  path: "/res714/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r715 = route({
  id: "res715.get",
  method: "GET",
  path: "/res715/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r716 = route({
  id: "res716.get",
  method: "GET",
  path: "/res716/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r717 = route({
  id: "res717.get",
  method: "GET",
  path: "/res717/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r718 = route({
  id: "res718.get",
  method: "GET",
  path: "/res718/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r719 = route({
  id: "res719.get",
  method: "GET",
  path: "/res719/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r720 = route({
  id: "res720.get",
  method: "GET",
  path: "/res720/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r721 = route({
  id: "res721.get",
  method: "GET",
  path: "/res721/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r722 = route({
  id: "res722.get",
  method: "GET",
  path: "/res722/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r723 = route({
  id: "res723.get",
  method: "GET",
  path: "/res723/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r724 = route({
  id: "res724.get",
  method: "GET",
  path: "/res724/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r725 = route({
  id: "res725.get",
  method: "GET",
  path: "/res725/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r726 = route({
  id: "res726.get",
  method: "GET",
  path: "/res726/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r727 = route({
  id: "res727.get",
  method: "GET",
  path: "/res727/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r728 = route({
  id: "res728.get",
  method: "GET",
  path: "/res728/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r729 = route({
  id: "res729.get",
  method: "GET",
  path: "/res729/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r730 = route({
  id: "res730.get",
  method: "GET",
  path: "/res730/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r731 = route({
  id: "res731.get",
  method: "GET",
  path: "/res731/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r732 = route({
  id: "res732.get",
  method: "GET",
  path: "/res732/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r733 = route({
  id: "res733.get",
  method: "GET",
  path: "/res733/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r734 = route({
  id: "res734.get",
  method: "GET",
  path: "/res734/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r735 = route({
  id: "res735.get",
  method: "GET",
  path: "/res735/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r736 = route({
  id: "res736.get",
  method: "GET",
  path: "/res736/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r737 = route({
  id: "res737.get",
  method: "GET",
  path: "/res737/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r738 = route({
  id: "res738.get",
  method: "GET",
  path: "/res738/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r739 = route({
  id: "res739.get",
  method: "GET",
  path: "/res739/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r740 = route({
  id: "res740.get",
  method: "GET",
  path: "/res740/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r741 = route({
  id: "res741.get",
  method: "GET",
  path: "/res741/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r742 = route({
  id: "res742.get",
  method: "GET",
  path: "/res742/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r743 = route({
  id: "res743.get",
  method: "GET",
  path: "/res743/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r744 = route({
  id: "res744.get",
  method: "GET",
  path: "/res744/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r745 = route({
  id: "res745.get",
  method: "GET",
  path: "/res745/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r746 = route({
  id: "res746.get",
  method: "GET",
  path: "/res746/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r747 = route({
  id: "res747.get",
  method: "GET",
  path: "/res747/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r748 = route({
  id: "res748.get",
  method: "GET",
  path: "/res748/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r749 = route({
  id: "res749.get",
  method: "GET",
  path: "/res749/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r750 = route({
  id: "res750.get",
  method: "GET",
  path: "/res750/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r751 = route({
  id: "res751.get",
  method: "GET",
  path: "/res751/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r752 = route({
  id: "res752.get",
  method: "GET",
  path: "/res752/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r753 = route({
  id: "res753.get",
  method: "GET",
  path: "/res753/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r754 = route({
  id: "res754.get",
  method: "GET",
  path: "/res754/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r755 = route({
  id: "res755.get",
  method: "GET",
  path: "/res755/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r756 = route({
  id: "res756.get",
  method: "GET",
  path: "/res756/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r757 = route({
  id: "res757.get",
  method: "GET",
  path: "/res757/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r758 = route({
  id: "res758.get",
  method: "GET",
  path: "/res758/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r759 = route({
  id: "res759.get",
  method: "GET",
  path: "/res759/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r760 = route({
  id: "res760.get",
  method: "GET",
  path: "/res760/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r761 = route({
  id: "res761.get",
  method: "GET",
  path: "/res761/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r762 = route({
  id: "res762.get",
  method: "GET",
  path: "/res762/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r763 = route({
  id: "res763.get",
  method: "GET",
  path: "/res763/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r764 = route({
  id: "res764.get",
  method: "GET",
  path: "/res764/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r765 = route({
  id: "res765.get",
  method: "GET",
  path: "/res765/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r766 = route({
  id: "res766.get",
  method: "GET",
  path: "/res766/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r767 = route({
  id: "res767.get",
  method: "GET",
  path: "/res767/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r768 = route({
  id: "res768.get",
  method: "GET",
  path: "/res768/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r769 = route({
  id: "res769.get",
  method: "GET",
  path: "/res769/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r770 = route({
  id: "res770.get",
  method: "GET",
  path: "/res770/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r771 = route({
  id: "res771.get",
  method: "GET",
  path: "/res771/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r772 = route({
  id: "res772.get",
  method: "GET",
  path: "/res772/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r773 = route({
  id: "res773.get",
  method: "GET",
  path: "/res773/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r774 = route({
  id: "res774.get",
  method: "GET",
  path: "/res774/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r775 = route({
  id: "res775.get",
  method: "GET",
  path: "/res775/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r776 = route({
  id: "res776.get",
  method: "GET",
  path: "/res776/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r777 = route({
  id: "res777.get",
  method: "GET",
  path: "/res777/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r778 = route({
  id: "res778.get",
  method: "GET",
  path: "/res778/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r779 = route({
  id: "res779.get",
  method: "GET",
  path: "/res779/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r780 = route({
  id: "res780.get",
  method: "GET",
  path: "/res780/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r781 = route({
  id: "res781.get",
  method: "GET",
  path: "/res781/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r782 = route({
  id: "res782.get",
  method: "GET",
  path: "/res782/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r783 = route({
  id: "res783.get",
  method: "GET",
  path: "/res783/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r784 = route({
  id: "res784.get",
  method: "GET",
  path: "/res784/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r785 = route({
  id: "res785.get",
  method: "GET",
  path: "/res785/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r786 = route({
  id: "res786.get",
  method: "GET",
  path: "/res786/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r787 = route({
  id: "res787.get",
  method: "GET",
  path: "/res787/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r788 = route({
  id: "res788.get",
  method: "GET",
  path: "/res788/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r789 = route({
  id: "res789.get",
  method: "GET",
  path: "/res789/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r790 = route({
  id: "res790.get",
  method: "GET",
  path: "/res790/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r791 = route({
  id: "res791.get",
  method: "GET",
  path: "/res791/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r792 = route({
  id: "res792.get",
  method: "GET",
  path: "/res792/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r793 = route({
  id: "res793.get",
  method: "GET",
  path: "/res793/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r794 = route({
  id: "res794.get",
  method: "GET",
  path: "/res794/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r795 = route({
  id: "res795.get",
  method: "GET",
  path: "/res795/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r796 = route({
  id: "res796.get",
  method: "GET",
  path: "/res796/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r797 = route({
  id: "res797.get",
  method: "GET",
  path: "/res797/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r798 = route({
  id: "res798.get",
  method: "GET",
  path: "/res798/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r799 = route({
  id: "res799.get",
  method: "GET",
  path: "/res799/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r800 = route({
  id: "res800.get",
  method: "GET",
  path: "/res800/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r801 = route({
  id: "res801.get",
  method: "GET",
  path: "/res801/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r802 = route({
  id: "res802.get",
  method: "GET",
  path: "/res802/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r803 = route({
  id: "res803.get",
  method: "GET",
  path: "/res803/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r804 = route({
  id: "res804.get",
  method: "GET",
  path: "/res804/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r805 = route({
  id: "res805.get",
  method: "GET",
  path: "/res805/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r806 = route({
  id: "res806.get",
  method: "GET",
  path: "/res806/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r807 = route({
  id: "res807.get",
  method: "GET",
  path: "/res807/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r808 = route({
  id: "res808.get",
  method: "GET",
  path: "/res808/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r809 = route({
  id: "res809.get",
  method: "GET",
  path: "/res809/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r810 = route({
  id: "res810.get",
  method: "GET",
  path: "/res810/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r811 = route({
  id: "res811.get",
  method: "GET",
  path: "/res811/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r812 = route({
  id: "res812.get",
  method: "GET",
  path: "/res812/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r813 = route({
  id: "res813.get",
  method: "GET",
  path: "/res813/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r814 = route({
  id: "res814.get",
  method: "GET",
  path: "/res814/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r815 = route({
  id: "res815.get",
  method: "GET",
  path: "/res815/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r816 = route({
  id: "res816.get",
  method: "GET",
  path: "/res816/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r817 = route({
  id: "res817.get",
  method: "GET",
  path: "/res817/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r818 = route({
  id: "res818.get",
  method: "GET",
  path: "/res818/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r819 = route({
  id: "res819.get",
  method: "GET",
  path: "/res819/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r820 = route({
  id: "res820.get",
  method: "GET",
  path: "/res820/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r821 = route({
  id: "res821.get",
  method: "GET",
  path: "/res821/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r822 = route({
  id: "res822.get",
  method: "GET",
  path: "/res822/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r823 = route({
  id: "res823.get",
  method: "GET",
  path: "/res823/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r824 = route({
  id: "res824.get",
  method: "GET",
  path: "/res824/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r825 = route({
  id: "res825.get",
  method: "GET",
  path: "/res825/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r826 = route({
  id: "res826.get",
  method: "GET",
  path: "/res826/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r827 = route({
  id: "res827.get",
  method: "GET",
  path: "/res827/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r828 = route({
  id: "res828.get",
  method: "GET",
  path: "/res828/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r829 = route({
  id: "res829.get",
  method: "GET",
  path: "/res829/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r830 = route({
  id: "res830.get",
  method: "GET",
  path: "/res830/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r831 = route({
  id: "res831.get",
  method: "GET",
  path: "/res831/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r832 = route({
  id: "res832.get",
  method: "GET",
  path: "/res832/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r833 = route({
  id: "res833.get",
  method: "GET",
  path: "/res833/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r834 = route({
  id: "res834.get",
  method: "GET",
  path: "/res834/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r835 = route({
  id: "res835.get",
  method: "GET",
  path: "/res835/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r836 = route({
  id: "res836.get",
  method: "GET",
  path: "/res836/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r837 = route({
  id: "res837.get",
  method: "GET",
  path: "/res837/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r838 = route({
  id: "res838.get",
  method: "GET",
  path: "/res838/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r839 = route({
  id: "res839.get",
  method: "GET",
  path: "/res839/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r840 = route({
  id: "res840.get",
  method: "GET",
  path: "/res840/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r841 = route({
  id: "res841.get",
  method: "GET",
  path: "/res841/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r842 = route({
  id: "res842.get",
  method: "GET",
  path: "/res842/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r843 = route({
  id: "res843.get",
  method: "GET",
  path: "/res843/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r844 = route({
  id: "res844.get",
  method: "GET",
  path: "/res844/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r845 = route({
  id: "res845.get",
  method: "GET",
  path: "/res845/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r846 = route({
  id: "res846.get",
  method: "GET",
  path: "/res846/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r847 = route({
  id: "res847.get",
  method: "GET",
  path: "/res847/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r848 = route({
  id: "res848.get",
  method: "GET",
  path: "/res848/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r849 = route({
  id: "res849.get",
  method: "GET",
  path: "/res849/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r850 = route({
  id: "res850.get",
  method: "GET",
  path: "/res850/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r851 = route({
  id: "res851.get",
  method: "GET",
  path: "/res851/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r852 = route({
  id: "res852.get",
  method: "GET",
  path: "/res852/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r853 = route({
  id: "res853.get",
  method: "GET",
  path: "/res853/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r854 = route({
  id: "res854.get",
  method: "GET",
  path: "/res854/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r855 = route({
  id: "res855.get",
  method: "GET",
  path: "/res855/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r856 = route({
  id: "res856.get",
  method: "GET",
  path: "/res856/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r857 = route({
  id: "res857.get",
  method: "GET",
  path: "/res857/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r858 = route({
  id: "res858.get",
  method: "GET",
  path: "/res858/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r859 = route({
  id: "res859.get",
  method: "GET",
  path: "/res859/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r860 = route({
  id: "res860.get",
  method: "GET",
  path: "/res860/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r861 = route({
  id: "res861.get",
  method: "GET",
  path: "/res861/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r862 = route({
  id: "res862.get",
  method: "GET",
  path: "/res862/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r863 = route({
  id: "res863.get",
  method: "GET",
  path: "/res863/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r864 = route({
  id: "res864.get",
  method: "GET",
  path: "/res864/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r865 = route({
  id: "res865.get",
  method: "GET",
  path: "/res865/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r866 = route({
  id: "res866.get",
  method: "GET",
  path: "/res866/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r867 = route({
  id: "res867.get",
  method: "GET",
  path: "/res867/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r868 = route({
  id: "res868.get",
  method: "GET",
  path: "/res868/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r869 = route({
  id: "res869.get",
  method: "GET",
  path: "/res869/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r870 = route({
  id: "res870.get",
  method: "GET",
  path: "/res870/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r871 = route({
  id: "res871.get",
  method: "GET",
  path: "/res871/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r872 = route({
  id: "res872.get",
  method: "GET",
  path: "/res872/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r873 = route({
  id: "res873.get",
  method: "GET",
  path: "/res873/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r874 = route({
  id: "res874.get",
  method: "GET",
  path: "/res874/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r875 = route({
  id: "res875.get",
  method: "GET",
  path: "/res875/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r876 = route({
  id: "res876.get",
  method: "GET",
  path: "/res876/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r877 = route({
  id: "res877.get",
  method: "GET",
  path: "/res877/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r878 = route({
  id: "res878.get",
  method: "GET",
  path: "/res878/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r879 = route({
  id: "res879.get",
  method: "GET",
  path: "/res879/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r880 = route({
  id: "res880.get",
  method: "GET",
  path: "/res880/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r881 = route({
  id: "res881.get",
  method: "GET",
  path: "/res881/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r882 = route({
  id: "res882.get",
  method: "GET",
  path: "/res882/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r883 = route({
  id: "res883.get",
  method: "GET",
  path: "/res883/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r884 = route({
  id: "res884.get",
  method: "GET",
  path: "/res884/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r885 = route({
  id: "res885.get",
  method: "GET",
  path: "/res885/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r886 = route({
  id: "res886.get",
  method: "GET",
  path: "/res886/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r887 = route({
  id: "res887.get",
  method: "GET",
  path: "/res887/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r888 = route({
  id: "res888.get",
  method: "GET",
  path: "/res888/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r889 = route({
  id: "res889.get",
  method: "GET",
  path: "/res889/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r890 = route({
  id: "res890.get",
  method: "GET",
  path: "/res890/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r891 = route({
  id: "res891.get",
  method: "GET",
  path: "/res891/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r892 = route({
  id: "res892.get",
  method: "GET",
  path: "/res892/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r893 = route({
  id: "res893.get",
  method: "GET",
  path: "/res893/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r894 = route({
  id: "res894.get",
  method: "GET",
  path: "/res894/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r895 = route({
  id: "res895.get",
  method: "GET",
  path: "/res895/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r896 = route({
  id: "res896.get",
  method: "GET",
  path: "/res896/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r897 = route({
  id: "res897.get",
  method: "GET",
  path: "/res897/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r898 = route({
  id: "res898.get",
  method: "GET",
  path: "/res898/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r899 = route({
  id: "res899.get",
  method: "GET",
  path: "/res899/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r900 = route({
  id: "res900.get",
  method: "GET",
  path: "/res900/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r901 = route({
  id: "res901.get",
  method: "GET",
  path: "/res901/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r902 = route({
  id: "res902.get",
  method: "GET",
  path: "/res902/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r903 = route({
  id: "res903.get",
  method: "GET",
  path: "/res903/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r904 = route({
  id: "res904.get",
  method: "GET",
  path: "/res904/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r905 = route({
  id: "res905.get",
  method: "GET",
  path: "/res905/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r906 = route({
  id: "res906.get",
  method: "GET",
  path: "/res906/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r907 = route({
  id: "res907.get",
  method: "GET",
  path: "/res907/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r908 = route({
  id: "res908.get",
  method: "GET",
  path: "/res908/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r909 = route({
  id: "res909.get",
  method: "GET",
  path: "/res909/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r910 = route({
  id: "res910.get",
  method: "GET",
  path: "/res910/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r911 = route({
  id: "res911.get",
  method: "GET",
  path: "/res911/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r912 = route({
  id: "res912.get",
  method: "GET",
  path: "/res912/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r913 = route({
  id: "res913.get",
  method: "GET",
  path: "/res913/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r914 = route({
  id: "res914.get",
  method: "GET",
  path: "/res914/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r915 = route({
  id: "res915.get",
  method: "GET",
  path: "/res915/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r916 = route({
  id: "res916.get",
  method: "GET",
  path: "/res916/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r917 = route({
  id: "res917.get",
  method: "GET",
  path: "/res917/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r918 = route({
  id: "res918.get",
  method: "GET",
  path: "/res918/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r919 = route({
  id: "res919.get",
  method: "GET",
  path: "/res919/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r920 = route({
  id: "res920.get",
  method: "GET",
  path: "/res920/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r921 = route({
  id: "res921.get",
  method: "GET",
  path: "/res921/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r922 = route({
  id: "res922.get",
  method: "GET",
  path: "/res922/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r923 = route({
  id: "res923.get",
  method: "GET",
  path: "/res923/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r924 = route({
  id: "res924.get",
  method: "GET",
  path: "/res924/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r925 = route({
  id: "res925.get",
  method: "GET",
  path: "/res925/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r926 = route({
  id: "res926.get",
  method: "GET",
  path: "/res926/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r927 = route({
  id: "res927.get",
  method: "GET",
  path: "/res927/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r928 = route({
  id: "res928.get",
  method: "GET",
  path: "/res928/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r929 = route({
  id: "res929.get",
  method: "GET",
  path: "/res929/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r930 = route({
  id: "res930.get",
  method: "GET",
  path: "/res930/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r931 = route({
  id: "res931.get",
  method: "GET",
  path: "/res931/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r932 = route({
  id: "res932.get",
  method: "GET",
  path: "/res932/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r933 = route({
  id: "res933.get",
  method: "GET",
  path: "/res933/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r934 = route({
  id: "res934.get",
  method: "GET",
  path: "/res934/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r935 = route({
  id: "res935.get",
  method: "GET",
  path: "/res935/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r936 = route({
  id: "res936.get",
  method: "GET",
  path: "/res936/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r937 = route({
  id: "res937.get",
  method: "GET",
  path: "/res937/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r938 = route({
  id: "res938.get",
  method: "GET",
  path: "/res938/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r939 = route({
  id: "res939.get",
  method: "GET",
  path: "/res939/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r940 = route({
  id: "res940.get",
  method: "GET",
  path: "/res940/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r941 = route({
  id: "res941.get",
  method: "GET",
  path: "/res941/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r942 = route({
  id: "res942.get",
  method: "GET",
  path: "/res942/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r943 = route({
  id: "res943.get",
  method: "GET",
  path: "/res943/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r944 = route({
  id: "res944.get",
  method: "GET",
  path: "/res944/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r945 = route({
  id: "res945.get",
  method: "GET",
  path: "/res945/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r946 = route({
  id: "res946.get",
  method: "GET",
  path: "/res946/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r947 = route({
  id: "res947.get",
  method: "GET",
  path: "/res947/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r948 = route({
  id: "res948.get",
  method: "GET",
  path: "/res948/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r949 = route({
  id: "res949.get",
  method: "GET",
  path: "/res949/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r950 = route({
  id: "res950.get",
  method: "GET",
  path: "/res950/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r951 = route({
  id: "res951.get",
  method: "GET",
  path: "/res951/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r952 = route({
  id: "res952.get",
  method: "GET",
  path: "/res952/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r953 = route({
  id: "res953.get",
  method: "GET",
  path: "/res953/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r954 = route({
  id: "res954.get",
  method: "GET",
  path: "/res954/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r955 = route({
  id: "res955.get",
  method: "GET",
  path: "/res955/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r956 = route({
  id: "res956.get",
  method: "GET",
  path: "/res956/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r957 = route({
  id: "res957.get",
  method: "GET",
  path: "/res957/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r958 = route({
  id: "res958.get",
  method: "GET",
  path: "/res958/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r959 = route({
  id: "res959.get",
  method: "GET",
  path: "/res959/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r960 = route({
  id: "res960.get",
  method: "GET",
  path: "/res960/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r961 = route({
  id: "res961.get",
  method: "GET",
  path: "/res961/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r962 = route({
  id: "res962.get",
  method: "GET",
  path: "/res962/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r963 = route({
  id: "res963.get",
  method: "GET",
  path: "/res963/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r964 = route({
  id: "res964.get",
  method: "GET",
  path: "/res964/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r965 = route({
  id: "res965.get",
  method: "GET",
  path: "/res965/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r966 = route({
  id: "res966.get",
  method: "GET",
  path: "/res966/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r967 = route({
  id: "res967.get",
  method: "GET",
  path: "/res967/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r968 = route({
  id: "res968.get",
  method: "GET",
  path: "/res968/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r969 = route({
  id: "res969.get",
  method: "GET",
  path: "/res969/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r970 = route({
  id: "res970.get",
  method: "GET",
  path: "/res970/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r971 = route({
  id: "res971.get",
  method: "GET",
  path: "/res971/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r972 = route({
  id: "res972.get",
  method: "GET",
  path: "/res972/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r973 = route({
  id: "res973.get",
  method: "GET",
  path: "/res973/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r974 = route({
  id: "res974.get",
  method: "GET",
  path: "/res974/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r975 = route({
  id: "res975.get",
  method: "GET",
  path: "/res975/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r976 = route({
  id: "res976.get",
  method: "GET",
  path: "/res976/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r977 = route({
  id: "res977.get",
  method: "GET",
  path: "/res977/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r978 = route({
  id: "res978.get",
  method: "GET",
  path: "/res978/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r979 = route({
  id: "res979.get",
  method: "GET",
  path: "/res979/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r980 = route({
  id: "res980.get",
  method: "GET",
  path: "/res980/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r981 = route({
  id: "res981.get",
  method: "GET",
  path: "/res981/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r982 = route({
  id: "res982.get",
  method: "GET",
  path: "/res982/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r983 = route({
  id: "res983.get",
  method: "GET",
  path: "/res983/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r984 = route({
  id: "res984.get",
  method: "GET",
  path: "/res984/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r985 = route({
  id: "res985.get",
  method: "GET",
  path: "/res985/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r986 = route({
  id: "res986.get",
  method: "GET",
  path: "/res986/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r987 = route({
  id: "res987.get",
  method: "GET",
  path: "/res987/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r988 = route({
  id: "res988.get",
  method: "GET",
  path: "/res988/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r989 = route({
  id: "res989.get",
  method: "GET",
  path: "/res989/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r990 = route({
  id: "res990.get",
  method: "GET",
  path: "/res990/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r991 = route({
  id: "res991.get",
  method: "GET",
  path: "/res991/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r992 = route({
  id: "res992.get",
  method: "GET",
  path: "/res992/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r993 = route({
  id: "res993.get",
  method: "GET",
  path: "/res993/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r994 = route({
  id: "res994.get",
  method: "GET",
  path: "/res994/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r995 = route({
  id: "res995.get",
  method: "GET",
  path: "/res995/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r996 = route({
  id: "res996.get",
  method: "GET",
  path: "/res996/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r997 = route({
  id: "res997.get",
  method: "GET",
  path: "/res997/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r998 = route({
  id: "res998.get",
  method: "GET",
  path: "/res998/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});
const r999 = route({
  id: "res999.get",
  method: "GET",
  path: "/res999/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 1000 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 1000 }),
});

export const app = defineApp({ id: "scale-1000", modules: [ defineModule({ id: "res", routes: [r0, r1, r2, r3, r4, r5, r6, r7, r8, r9, r10, r11, r12, r13, r14, r15, r16, r17, r18, r19, r20, r21, r22, r23, r24, r25, r26, r27, r28, r29, r30, r31, r32, r33, r34, r35, r36, r37, r38, r39, r40, r41, r42, r43, r44, r45, r46, r47, r48, r49, r50, r51, r52, r53, r54, r55, r56, r57, r58, r59, r60, r61, r62, r63, r64, r65, r66, r67, r68, r69, r70, r71, r72, r73, r74, r75, r76, r77, r78, r79, r80, r81, r82, r83, r84, r85, r86, r87, r88, r89, r90, r91, r92, r93, r94, r95, r96, r97, r98, r99, r100, r101, r102, r103, r104, r105, r106, r107, r108, r109, r110, r111, r112, r113, r114, r115, r116, r117, r118, r119, r120, r121, r122, r123, r124, r125, r126, r127, r128, r129, r130, r131, r132, r133, r134, r135, r136, r137, r138, r139, r140, r141, r142, r143, r144, r145, r146, r147, r148, r149, r150, r151, r152, r153, r154, r155, r156, r157, r158, r159, r160, r161, r162, r163, r164, r165, r166, r167, r168, r169, r170, r171, r172, r173, r174, r175, r176, r177, r178, r179, r180, r181, r182, r183, r184, r185, r186, r187, r188, r189, r190, r191, r192, r193, r194, r195, r196, r197, r198, r199, r200, r201, r202, r203, r204, r205, r206, r207, r208, r209, r210, r211, r212, r213, r214, r215, r216, r217, r218, r219, r220, r221, r222, r223, r224, r225, r226, r227, r228, r229, r230, r231, r232, r233, r234, r235, r236, r237, r238, r239, r240, r241, r242, r243, r244, r245, r246, r247, r248, r249, r250, r251, r252, r253, r254, r255, r256, r257, r258, r259, r260, r261, r262, r263, r264, r265, r266, r267, r268, r269, r270, r271, r272, r273, r274, r275, r276, r277, r278, r279, r280, r281, r282, r283, r284, r285, r286, r287, r288, r289, r290, r291, r292, r293, r294, r295, r296, r297, r298, r299, r300, r301, r302, r303, r304, r305, r306, r307, r308, r309, r310, r311, r312, r313, r314, r315, r316, r317, r318, r319, r320, r321, r322, r323, r324, r325, r326, r327, r328, r329, r330, r331, r332, r333, r334, r335, r336, r337, r338, r339, r340, r341, r342, r343, r344, r345, r346, r347, r348, r349, r350, r351, r352, r353, r354, r355, r356, r357, r358, r359, r360, r361, r362, r363, r364, r365, r366, r367, r368, r369, r370, r371, r372, r373, r374, r375, r376, r377, r378, r379, r380, r381, r382, r383, r384, r385, r386, r387, r388, r389, r390, r391, r392, r393, r394, r395, r396, r397, r398, r399, r400, r401, r402, r403, r404, r405, r406, r407, r408, r409, r410, r411, r412, r413, r414, r415, r416, r417, r418, r419, r420, r421, r422, r423, r424, r425, r426, r427, r428, r429, r430, r431, r432, r433, r434, r435, r436, r437, r438, r439, r440, r441, r442, r443, r444, r445, r446, r447, r448, r449, r450, r451, r452, r453, r454, r455, r456, r457, r458, r459, r460, r461, r462, r463, r464, r465, r466, r467, r468, r469, r470, r471, r472, r473, r474, r475, r476, r477, r478, r479, r480, r481, r482, r483, r484, r485, r486, r487, r488, r489, r490, r491, r492, r493, r494, r495, r496, r497, r498, r499, r500, r501, r502, r503, r504, r505, r506, r507, r508, r509, r510, r511, r512, r513, r514, r515, r516, r517, r518, r519, r520, r521, r522, r523, r524, r525, r526, r527, r528, r529, r530, r531, r532, r533, r534, r535, r536, r537, r538, r539, r540, r541, r542, r543, r544, r545, r546, r547, r548, r549, r550, r551, r552, r553, r554, r555, r556, r557, r558, r559, r560, r561, r562, r563, r564, r565, r566, r567, r568, r569, r570, r571, r572, r573, r574, r575, r576, r577, r578, r579, r580, r581, r582, r583, r584, r585, r586, r587, r588, r589, r590, r591, r592, r593, r594, r595, r596, r597, r598, r599, r600, r601, r602, r603, r604, r605, r606, r607, r608, r609, r610, r611, r612, r613, r614, r615, r616, r617, r618, r619, r620, r621, r622, r623, r624, r625, r626, r627, r628, r629, r630, r631, r632, r633, r634, r635, r636, r637, r638, r639, r640, r641, r642, r643, r644, r645, r646, r647, r648, r649, r650, r651, r652, r653, r654, r655, r656, r657, r658, r659, r660, r661, r662, r663, r664, r665, r666, r667, r668, r669, r670, r671, r672, r673, r674, r675, r676, r677, r678, r679, r680, r681, r682, r683, r684, r685, r686, r687, r688, r689, r690, r691, r692, r693, r694, r695, r696, r697, r698, r699, r700, r701, r702, r703, r704, r705, r706, r707, r708, r709, r710, r711, r712, r713, r714, r715, r716, r717, r718, r719, r720, r721, r722, r723, r724, r725, r726, r727, r728, r729, r730, r731, r732, r733, r734, r735, r736, r737, r738, r739, r740, r741, r742, r743, r744, r745, r746, r747, r748, r749, r750, r751, r752, r753, r754, r755, r756, r757, r758, r759, r760, r761, r762, r763, r764, r765, r766, r767, r768, r769, r770, r771, r772, r773, r774, r775, r776, r777, r778, r779, r780, r781, r782, r783, r784, r785, r786, r787, r788, r789, r790, r791, r792, r793, r794, r795, r796, r797, r798, r799, r800, r801, r802, r803, r804, r805, r806, r807, r808, r809, r810, r811, r812, r813, r814, r815, r816, r817, r818, r819, r820, r821, r822, r823, r824, r825, r826, r827, r828, r829, r830, r831, r832, r833, r834, r835, r836, r837, r838, r839, r840, r841, r842, r843, r844, r845, r846, r847, r848, r849, r850, r851, r852, r853, r854, r855, r856, r857, r858, r859, r860, r861, r862, r863, r864, r865, r866, r867, r868, r869, r870, r871, r872, r873, r874, r875, r876, r877, r878, r879, r880, r881, r882, r883, r884, r885, r886, r887, r888, r889, r890, r891, r892, r893, r894, r895, r896, r897, r898, r899, r900, r901, r902, r903, r904, r905, r906, r907, r908, r909, r910, r911, r912, r913, r914, r915, r916, r917, r918, r919, r920, r921, r922, r923, r924, r925, r926, r927, r928, r929, r930, r931, r932, r933, r934, r935, r936, r937, r938, r939, r940, r941, r942, r943, r944, r945, r946, r947, r948, r949, r950, r951, r952, r953, r954, r955, r956, r957, r958, r959, r960, r961, r962, r963, r964, r965, r966, r967, r968, r969, r970, r971, r972, r973, r974, r975, r976, r977, r978, r979, r980, r981, r982, r983, r984, r985, r986, r987, r988, r989, r990, r991, r992, r993, r994, r995, r996, r997, r998, r999] }) ] });
