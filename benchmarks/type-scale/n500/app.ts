import { defineApp, defineModule, route } from "@q/core";
import { s } from "@q/schema";

const r0 = route({
  id: "res0.get",
  method: "GET",
  path: "/res0/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r1 = route({
  id: "res1.get",
  method: "GET",
  path: "/res1/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r2 = route({
  id: "res2.get",
  method: "GET",
  path: "/res2/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r3 = route({
  id: "res3.get",
  method: "GET",
  path: "/res3/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r4 = route({
  id: "res4.get",
  method: "GET",
  path: "/res4/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r5 = route({
  id: "res5.get",
  method: "GET",
  path: "/res5/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r6 = route({
  id: "res6.get",
  method: "GET",
  path: "/res6/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r7 = route({
  id: "res7.get",
  method: "GET",
  path: "/res7/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r8 = route({
  id: "res8.get",
  method: "GET",
  path: "/res8/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r9 = route({
  id: "res9.get",
  method: "GET",
  path: "/res9/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r10 = route({
  id: "res10.get",
  method: "GET",
  path: "/res10/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r11 = route({
  id: "res11.get",
  method: "GET",
  path: "/res11/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r12 = route({
  id: "res12.get",
  method: "GET",
  path: "/res12/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r13 = route({
  id: "res13.get",
  method: "GET",
  path: "/res13/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r14 = route({
  id: "res14.get",
  method: "GET",
  path: "/res14/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r15 = route({
  id: "res15.get",
  method: "GET",
  path: "/res15/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r16 = route({
  id: "res16.get",
  method: "GET",
  path: "/res16/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r17 = route({
  id: "res17.get",
  method: "GET",
  path: "/res17/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r18 = route({
  id: "res18.get",
  method: "GET",
  path: "/res18/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r19 = route({
  id: "res19.get",
  method: "GET",
  path: "/res19/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r20 = route({
  id: "res20.get",
  method: "GET",
  path: "/res20/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r21 = route({
  id: "res21.get",
  method: "GET",
  path: "/res21/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r22 = route({
  id: "res22.get",
  method: "GET",
  path: "/res22/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r23 = route({
  id: "res23.get",
  method: "GET",
  path: "/res23/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r24 = route({
  id: "res24.get",
  method: "GET",
  path: "/res24/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r25 = route({
  id: "res25.get",
  method: "GET",
  path: "/res25/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r26 = route({
  id: "res26.get",
  method: "GET",
  path: "/res26/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r27 = route({
  id: "res27.get",
  method: "GET",
  path: "/res27/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r28 = route({
  id: "res28.get",
  method: "GET",
  path: "/res28/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r29 = route({
  id: "res29.get",
  method: "GET",
  path: "/res29/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r30 = route({
  id: "res30.get",
  method: "GET",
  path: "/res30/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r31 = route({
  id: "res31.get",
  method: "GET",
  path: "/res31/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r32 = route({
  id: "res32.get",
  method: "GET",
  path: "/res32/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r33 = route({
  id: "res33.get",
  method: "GET",
  path: "/res33/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r34 = route({
  id: "res34.get",
  method: "GET",
  path: "/res34/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r35 = route({
  id: "res35.get",
  method: "GET",
  path: "/res35/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r36 = route({
  id: "res36.get",
  method: "GET",
  path: "/res36/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r37 = route({
  id: "res37.get",
  method: "GET",
  path: "/res37/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r38 = route({
  id: "res38.get",
  method: "GET",
  path: "/res38/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r39 = route({
  id: "res39.get",
  method: "GET",
  path: "/res39/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r40 = route({
  id: "res40.get",
  method: "GET",
  path: "/res40/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r41 = route({
  id: "res41.get",
  method: "GET",
  path: "/res41/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r42 = route({
  id: "res42.get",
  method: "GET",
  path: "/res42/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r43 = route({
  id: "res43.get",
  method: "GET",
  path: "/res43/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r44 = route({
  id: "res44.get",
  method: "GET",
  path: "/res44/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r45 = route({
  id: "res45.get",
  method: "GET",
  path: "/res45/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r46 = route({
  id: "res46.get",
  method: "GET",
  path: "/res46/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r47 = route({
  id: "res47.get",
  method: "GET",
  path: "/res47/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r48 = route({
  id: "res48.get",
  method: "GET",
  path: "/res48/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r49 = route({
  id: "res49.get",
  method: "GET",
  path: "/res49/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r50 = route({
  id: "res50.get",
  method: "GET",
  path: "/res50/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r51 = route({
  id: "res51.get",
  method: "GET",
  path: "/res51/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r52 = route({
  id: "res52.get",
  method: "GET",
  path: "/res52/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r53 = route({
  id: "res53.get",
  method: "GET",
  path: "/res53/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r54 = route({
  id: "res54.get",
  method: "GET",
  path: "/res54/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r55 = route({
  id: "res55.get",
  method: "GET",
  path: "/res55/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r56 = route({
  id: "res56.get",
  method: "GET",
  path: "/res56/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r57 = route({
  id: "res57.get",
  method: "GET",
  path: "/res57/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r58 = route({
  id: "res58.get",
  method: "GET",
  path: "/res58/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r59 = route({
  id: "res59.get",
  method: "GET",
  path: "/res59/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r60 = route({
  id: "res60.get",
  method: "GET",
  path: "/res60/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r61 = route({
  id: "res61.get",
  method: "GET",
  path: "/res61/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r62 = route({
  id: "res62.get",
  method: "GET",
  path: "/res62/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r63 = route({
  id: "res63.get",
  method: "GET",
  path: "/res63/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r64 = route({
  id: "res64.get",
  method: "GET",
  path: "/res64/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r65 = route({
  id: "res65.get",
  method: "GET",
  path: "/res65/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r66 = route({
  id: "res66.get",
  method: "GET",
  path: "/res66/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r67 = route({
  id: "res67.get",
  method: "GET",
  path: "/res67/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r68 = route({
  id: "res68.get",
  method: "GET",
  path: "/res68/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r69 = route({
  id: "res69.get",
  method: "GET",
  path: "/res69/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r70 = route({
  id: "res70.get",
  method: "GET",
  path: "/res70/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r71 = route({
  id: "res71.get",
  method: "GET",
  path: "/res71/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r72 = route({
  id: "res72.get",
  method: "GET",
  path: "/res72/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r73 = route({
  id: "res73.get",
  method: "GET",
  path: "/res73/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r74 = route({
  id: "res74.get",
  method: "GET",
  path: "/res74/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r75 = route({
  id: "res75.get",
  method: "GET",
  path: "/res75/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r76 = route({
  id: "res76.get",
  method: "GET",
  path: "/res76/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r77 = route({
  id: "res77.get",
  method: "GET",
  path: "/res77/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r78 = route({
  id: "res78.get",
  method: "GET",
  path: "/res78/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r79 = route({
  id: "res79.get",
  method: "GET",
  path: "/res79/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r80 = route({
  id: "res80.get",
  method: "GET",
  path: "/res80/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r81 = route({
  id: "res81.get",
  method: "GET",
  path: "/res81/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r82 = route({
  id: "res82.get",
  method: "GET",
  path: "/res82/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r83 = route({
  id: "res83.get",
  method: "GET",
  path: "/res83/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r84 = route({
  id: "res84.get",
  method: "GET",
  path: "/res84/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r85 = route({
  id: "res85.get",
  method: "GET",
  path: "/res85/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r86 = route({
  id: "res86.get",
  method: "GET",
  path: "/res86/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r87 = route({
  id: "res87.get",
  method: "GET",
  path: "/res87/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r88 = route({
  id: "res88.get",
  method: "GET",
  path: "/res88/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r89 = route({
  id: "res89.get",
  method: "GET",
  path: "/res89/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r90 = route({
  id: "res90.get",
  method: "GET",
  path: "/res90/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r91 = route({
  id: "res91.get",
  method: "GET",
  path: "/res91/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r92 = route({
  id: "res92.get",
  method: "GET",
  path: "/res92/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r93 = route({
  id: "res93.get",
  method: "GET",
  path: "/res93/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r94 = route({
  id: "res94.get",
  method: "GET",
  path: "/res94/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r95 = route({
  id: "res95.get",
  method: "GET",
  path: "/res95/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r96 = route({
  id: "res96.get",
  method: "GET",
  path: "/res96/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r97 = route({
  id: "res97.get",
  method: "GET",
  path: "/res97/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r98 = route({
  id: "res98.get",
  method: "GET",
  path: "/res98/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r99 = route({
  id: "res99.get",
  method: "GET",
  path: "/res99/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r100 = route({
  id: "res100.get",
  method: "GET",
  path: "/res100/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r101 = route({
  id: "res101.get",
  method: "GET",
  path: "/res101/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r102 = route({
  id: "res102.get",
  method: "GET",
  path: "/res102/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r103 = route({
  id: "res103.get",
  method: "GET",
  path: "/res103/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r104 = route({
  id: "res104.get",
  method: "GET",
  path: "/res104/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r105 = route({
  id: "res105.get",
  method: "GET",
  path: "/res105/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r106 = route({
  id: "res106.get",
  method: "GET",
  path: "/res106/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r107 = route({
  id: "res107.get",
  method: "GET",
  path: "/res107/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r108 = route({
  id: "res108.get",
  method: "GET",
  path: "/res108/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r109 = route({
  id: "res109.get",
  method: "GET",
  path: "/res109/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r110 = route({
  id: "res110.get",
  method: "GET",
  path: "/res110/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r111 = route({
  id: "res111.get",
  method: "GET",
  path: "/res111/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r112 = route({
  id: "res112.get",
  method: "GET",
  path: "/res112/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r113 = route({
  id: "res113.get",
  method: "GET",
  path: "/res113/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r114 = route({
  id: "res114.get",
  method: "GET",
  path: "/res114/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r115 = route({
  id: "res115.get",
  method: "GET",
  path: "/res115/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r116 = route({
  id: "res116.get",
  method: "GET",
  path: "/res116/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r117 = route({
  id: "res117.get",
  method: "GET",
  path: "/res117/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r118 = route({
  id: "res118.get",
  method: "GET",
  path: "/res118/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r119 = route({
  id: "res119.get",
  method: "GET",
  path: "/res119/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r120 = route({
  id: "res120.get",
  method: "GET",
  path: "/res120/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r121 = route({
  id: "res121.get",
  method: "GET",
  path: "/res121/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r122 = route({
  id: "res122.get",
  method: "GET",
  path: "/res122/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r123 = route({
  id: "res123.get",
  method: "GET",
  path: "/res123/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r124 = route({
  id: "res124.get",
  method: "GET",
  path: "/res124/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r125 = route({
  id: "res125.get",
  method: "GET",
  path: "/res125/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r126 = route({
  id: "res126.get",
  method: "GET",
  path: "/res126/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r127 = route({
  id: "res127.get",
  method: "GET",
  path: "/res127/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r128 = route({
  id: "res128.get",
  method: "GET",
  path: "/res128/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r129 = route({
  id: "res129.get",
  method: "GET",
  path: "/res129/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r130 = route({
  id: "res130.get",
  method: "GET",
  path: "/res130/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r131 = route({
  id: "res131.get",
  method: "GET",
  path: "/res131/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r132 = route({
  id: "res132.get",
  method: "GET",
  path: "/res132/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r133 = route({
  id: "res133.get",
  method: "GET",
  path: "/res133/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r134 = route({
  id: "res134.get",
  method: "GET",
  path: "/res134/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r135 = route({
  id: "res135.get",
  method: "GET",
  path: "/res135/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r136 = route({
  id: "res136.get",
  method: "GET",
  path: "/res136/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r137 = route({
  id: "res137.get",
  method: "GET",
  path: "/res137/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r138 = route({
  id: "res138.get",
  method: "GET",
  path: "/res138/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r139 = route({
  id: "res139.get",
  method: "GET",
  path: "/res139/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r140 = route({
  id: "res140.get",
  method: "GET",
  path: "/res140/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r141 = route({
  id: "res141.get",
  method: "GET",
  path: "/res141/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r142 = route({
  id: "res142.get",
  method: "GET",
  path: "/res142/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r143 = route({
  id: "res143.get",
  method: "GET",
  path: "/res143/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r144 = route({
  id: "res144.get",
  method: "GET",
  path: "/res144/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r145 = route({
  id: "res145.get",
  method: "GET",
  path: "/res145/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r146 = route({
  id: "res146.get",
  method: "GET",
  path: "/res146/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r147 = route({
  id: "res147.get",
  method: "GET",
  path: "/res147/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r148 = route({
  id: "res148.get",
  method: "GET",
  path: "/res148/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r149 = route({
  id: "res149.get",
  method: "GET",
  path: "/res149/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r150 = route({
  id: "res150.get",
  method: "GET",
  path: "/res150/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r151 = route({
  id: "res151.get",
  method: "GET",
  path: "/res151/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r152 = route({
  id: "res152.get",
  method: "GET",
  path: "/res152/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r153 = route({
  id: "res153.get",
  method: "GET",
  path: "/res153/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r154 = route({
  id: "res154.get",
  method: "GET",
  path: "/res154/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r155 = route({
  id: "res155.get",
  method: "GET",
  path: "/res155/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r156 = route({
  id: "res156.get",
  method: "GET",
  path: "/res156/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r157 = route({
  id: "res157.get",
  method: "GET",
  path: "/res157/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r158 = route({
  id: "res158.get",
  method: "GET",
  path: "/res158/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r159 = route({
  id: "res159.get",
  method: "GET",
  path: "/res159/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r160 = route({
  id: "res160.get",
  method: "GET",
  path: "/res160/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r161 = route({
  id: "res161.get",
  method: "GET",
  path: "/res161/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r162 = route({
  id: "res162.get",
  method: "GET",
  path: "/res162/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r163 = route({
  id: "res163.get",
  method: "GET",
  path: "/res163/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r164 = route({
  id: "res164.get",
  method: "GET",
  path: "/res164/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r165 = route({
  id: "res165.get",
  method: "GET",
  path: "/res165/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r166 = route({
  id: "res166.get",
  method: "GET",
  path: "/res166/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r167 = route({
  id: "res167.get",
  method: "GET",
  path: "/res167/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r168 = route({
  id: "res168.get",
  method: "GET",
  path: "/res168/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r169 = route({
  id: "res169.get",
  method: "GET",
  path: "/res169/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r170 = route({
  id: "res170.get",
  method: "GET",
  path: "/res170/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r171 = route({
  id: "res171.get",
  method: "GET",
  path: "/res171/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r172 = route({
  id: "res172.get",
  method: "GET",
  path: "/res172/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r173 = route({
  id: "res173.get",
  method: "GET",
  path: "/res173/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r174 = route({
  id: "res174.get",
  method: "GET",
  path: "/res174/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r175 = route({
  id: "res175.get",
  method: "GET",
  path: "/res175/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r176 = route({
  id: "res176.get",
  method: "GET",
  path: "/res176/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r177 = route({
  id: "res177.get",
  method: "GET",
  path: "/res177/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r178 = route({
  id: "res178.get",
  method: "GET",
  path: "/res178/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r179 = route({
  id: "res179.get",
  method: "GET",
  path: "/res179/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r180 = route({
  id: "res180.get",
  method: "GET",
  path: "/res180/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r181 = route({
  id: "res181.get",
  method: "GET",
  path: "/res181/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r182 = route({
  id: "res182.get",
  method: "GET",
  path: "/res182/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r183 = route({
  id: "res183.get",
  method: "GET",
  path: "/res183/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r184 = route({
  id: "res184.get",
  method: "GET",
  path: "/res184/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r185 = route({
  id: "res185.get",
  method: "GET",
  path: "/res185/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r186 = route({
  id: "res186.get",
  method: "GET",
  path: "/res186/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r187 = route({
  id: "res187.get",
  method: "GET",
  path: "/res187/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r188 = route({
  id: "res188.get",
  method: "GET",
  path: "/res188/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r189 = route({
  id: "res189.get",
  method: "GET",
  path: "/res189/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r190 = route({
  id: "res190.get",
  method: "GET",
  path: "/res190/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r191 = route({
  id: "res191.get",
  method: "GET",
  path: "/res191/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r192 = route({
  id: "res192.get",
  method: "GET",
  path: "/res192/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r193 = route({
  id: "res193.get",
  method: "GET",
  path: "/res193/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r194 = route({
  id: "res194.get",
  method: "GET",
  path: "/res194/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r195 = route({
  id: "res195.get",
  method: "GET",
  path: "/res195/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r196 = route({
  id: "res196.get",
  method: "GET",
  path: "/res196/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r197 = route({
  id: "res197.get",
  method: "GET",
  path: "/res197/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r198 = route({
  id: "res198.get",
  method: "GET",
  path: "/res198/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r199 = route({
  id: "res199.get",
  method: "GET",
  path: "/res199/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r200 = route({
  id: "res200.get",
  method: "GET",
  path: "/res200/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r201 = route({
  id: "res201.get",
  method: "GET",
  path: "/res201/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r202 = route({
  id: "res202.get",
  method: "GET",
  path: "/res202/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r203 = route({
  id: "res203.get",
  method: "GET",
  path: "/res203/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r204 = route({
  id: "res204.get",
  method: "GET",
  path: "/res204/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r205 = route({
  id: "res205.get",
  method: "GET",
  path: "/res205/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r206 = route({
  id: "res206.get",
  method: "GET",
  path: "/res206/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r207 = route({
  id: "res207.get",
  method: "GET",
  path: "/res207/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r208 = route({
  id: "res208.get",
  method: "GET",
  path: "/res208/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r209 = route({
  id: "res209.get",
  method: "GET",
  path: "/res209/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r210 = route({
  id: "res210.get",
  method: "GET",
  path: "/res210/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r211 = route({
  id: "res211.get",
  method: "GET",
  path: "/res211/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r212 = route({
  id: "res212.get",
  method: "GET",
  path: "/res212/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r213 = route({
  id: "res213.get",
  method: "GET",
  path: "/res213/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r214 = route({
  id: "res214.get",
  method: "GET",
  path: "/res214/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r215 = route({
  id: "res215.get",
  method: "GET",
  path: "/res215/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r216 = route({
  id: "res216.get",
  method: "GET",
  path: "/res216/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r217 = route({
  id: "res217.get",
  method: "GET",
  path: "/res217/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r218 = route({
  id: "res218.get",
  method: "GET",
  path: "/res218/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r219 = route({
  id: "res219.get",
  method: "GET",
  path: "/res219/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r220 = route({
  id: "res220.get",
  method: "GET",
  path: "/res220/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r221 = route({
  id: "res221.get",
  method: "GET",
  path: "/res221/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r222 = route({
  id: "res222.get",
  method: "GET",
  path: "/res222/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r223 = route({
  id: "res223.get",
  method: "GET",
  path: "/res223/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r224 = route({
  id: "res224.get",
  method: "GET",
  path: "/res224/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r225 = route({
  id: "res225.get",
  method: "GET",
  path: "/res225/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r226 = route({
  id: "res226.get",
  method: "GET",
  path: "/res226/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r227 = route({
  id: "res227.get",
  method: "GET",
  path: "/res227/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r228 = route({
  id: "res228.get",
  method: "GET",
  path: "/res228/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r229 = route({
  id: "res229.get",
  method: "GET",
  path: "/res229/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r230 = route({
  id: "res230.get",
  method: "GET",
  path: "/res230/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r231 = route({
  id: "res231.get",
  method: "GET",
  path: "/res231/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r232 = route({
  id: "res232.get",
  method: "GET",
  path: "/res232/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r233 = route({
  id: "res233.get",
  method: "GET",
  path: "/res233/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r234 = route({
  id: "res234.get",
  method: "GET",
  path: "/res234/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r235 = route({
  id: "res235.get",
  method: "GET",
  path: "/res235/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r236 = route({
  id: "res236.get",
  method: "GET",
  path: "/res236/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r237 = route({
  id: "res237.get",
  method: "GET",
  path: "/res237/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r238 = route({
  id: "res238.get",
  method: "GET",
  path: "/res238/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r239 = route({
  id: "res239.get",
  method: "GET",
  path: "/res239/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r240 = route({
  id: "res240.get",
  method: "GET",
  path: "/res240/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r241 = route({
  id: "res241.get",
  method: "GET",
  path: "/res241/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r242 = route({
  id: "res242.get",
  method: "GET",
  path: "/res242/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r243 = route({
  id: "res243.get",
  method: "GET",
  path: "/res243/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r244 = route({
  id: "res244.get",
  method: "GET",
  path: "/res244/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r245 = route({
  id: "res245.get",
  method: "GET",
  path: "/res245/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r246 = route({
  id: "res246.get",
  method: "GET",
  path: "/res246/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r247 = route({
  id: "res247.get",
  method: "GET",
  path: "/res247/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r248 = route({
  id: "res248.get",
  method: "GET",
  path: "/res248/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r249 = route({
  id: "res249.get",
  method: "GET",
  path: "/res249/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r250 = route({
  id: "res250.get",
  method: "GET",
  path: "/res250/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r251 = route({
  id: "res251.get",
  method: "GET",
  path: "/res251/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r252 = route({
  id: "res252.get",
  method: "GET",
  path: "/res252/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r253 = route({
  id: "res253.get",
  method: "GET",
  path: "/res253/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r254 = route({
  id: "res254.get",
  method: "GET",
  path: "/res254/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r255 = route({
  id: "res255.get",
  method: "GET",
  path: "/res255/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r256 = route({
  id: "res256.get",
  method: "GET",
  path: "/res256/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r257 = route({
  id: "res257.get",
  method: "GET",
  path: "/res257/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r258 = route({
  id: "res258.get",
  method: "GET",
  path: "/res258/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r259 = route({
  id: "res259.get",
  method: "GET",
  path: "/res259/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r260 = route({
  id: "res260.get",
  method: "GET",
  path: "/res260/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r261 = route({
  id: "res261.get",
  method: "GET",
  path: "/res261/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r262 = route({
  id: "res262.get",
  method: "GET",
  path: "/res262/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r263 = route({
  id: "res263.get",
  method: "GET",
  path: "/res263/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r264 = route({
  id: "res264.get",
  method: "GET",
  path: "/res264/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r265 = route({
  id: "res265.get",
  method: "GET",
  path: "/res265/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r266 = route({
  id: "res266.get",
  method: "GET",
  path: "/res266/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r267 = route({
  id: "res267.get",
  method: "GET",
  path: "/res267/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r268 = route({
  id: "res268.get",
  method: "GET",
  path: "/res268/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r269 = route({
  id: "res269.get",
  method: "GET",
  path: "/res269/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r270 = route({
  id: "res270.get",
  method: "GET",
  path: "/res270/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r271 = route({
  id: "res271.get",
  method: "GET",
  path: "/res271/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r272 = route({
  id: "res272.get",
  method: "GET",
  path: "/res272/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r273 = route({
  id: "res273.get",
  method: "GET",
  path: "/res273/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r274 = route({
  id: "res274.get",
  method: "GET",
  path: "/res274/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r275 = route({
  id: "res275.get",
  method: "GET",
  path: "/res275/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r276 = route({
  id: "res276.get",
  method: "GET",
  path: "/res276/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r277 = route({
  id: "res277.get",
  method: "GET",
  path: "/res277/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r278 = route({
  id: "res278.get",
  method: "GET",
  path: "/res278/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r279 = route({
  id: "res279.get",
  method: "GET",
  path: "/res279/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r280 = route({
  id: "res280.get",
  method: "GET",
  path: "/res280/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r281 = route({
  id: "res281.get",
  method: "GET",
  path: "/res281/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r282 = route({
  id: "res282.get",
  method: "GET",
  path: "/res282/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r283 = route({
  id: "res283.get",
  method: "GET",
  path: "/res283/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r284 = route({
  id: "res284.get",
  method: "GET",
  path: "/res284/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r285 = route({
  id: "res285.get",
  method: "GET",
  path: "/res285/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r286 = route({
  id: "res286.get",
  method: "GET",
  path: "/res286/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r287 = route({
  id: "res287.get",
  method: "GET",
  path: "/res287/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r288 = route({
  id: "res288.get",
  method: "GET",
  path: "/res288/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r289 = route({
  id: "res289.get",
  method: "GET",
  path: "/res289/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r290 = route({
  id: "res290.get",
  method: "GET",
  path: "/res290/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r291 = route({
  id: "res291.get",
  method: "GET",
  path: "/res291/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r292 = route({
  id: "res292.get",
  method: "GET",
  path: "/res292/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r293 = route({
  id: "res293.get",
  method: "GET",
  path: "/res293/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r294 = route({
  id: "res294.get",
  method: "GET",
  path: "/res294/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r295 = route({
  id: "res295.get",
  method: "GET",
  path: "/res295/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r296 = route({
  id: "res296.get",
  method: "GET",
  path: "/res296/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r297 = route({
  id: "res297.get",
  method: "GET",
  path: "/res297/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r298 = route({
  id: "res298.get",
  method: "GET",
  path: "/res298/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r299 = route({
  id: "res299.get",
  method: "GET",
  path: "/res299/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r300 = route({
  id: "res300.get",
  method: "GET",
  path: "/res300/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r301 = route({
  id: "res301.get",
  method: "GET",
  path: "/res301/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r302 = route({
  id: "res302.get",
  method: "GET",
  path: "/res302/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r303 = route({
  id: "res303.get",
  method: "GET",
  path: "/res303/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r304 = route({
  id: "res304.get",
  method: "GET",
  path: "/res304/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r305 = route({
  id: "res305.get",
  method: "GET",
  path: "/res305/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r306 = route({
  id: "res306.get",
  method: "GET",
  path: "/res306/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r307 = route({
  id: "res307.get",
  method: "GET",
  path: "/res307/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r308 = route({
  id: "res308.get",
  method: "GET",
  path: "/res308/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r309 = route({
  id: "res309.get",
  method: "GET",
  path: "/res309/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r310 = route({
  id: "res310.get",
  method: "GET",
  path: "/res310/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r311 = route({
  id: "res311.get",
  method: "GET",
  path: "/res311/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r312 = route({
  id: "res312.get",
  method: "GET",
  path: "/res312/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r313 = route({
  id: "res313.get",
  method: "GET",
  path: "/res313/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r314 = route({
  id: "res314.get",
  method: "GET",
  path: "/res314/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r315 = route({
  id: "res315.get",
  method: "GET",
  path: "/res315/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r316 = route({
  id: "res316.get",
  method: "GET",
  path: "/res316/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r317 = route({
  id: "res317.get",
  method: "GET",
  path: "/res317/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r318 = route({
  id: "res318.get",
  method: "GET",
  path: "/res318/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r319 = route({
  id: "res319.get",
  method: "GET",
  path: "/res319/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r320 = route({
  id: "res320.get",
  method: "GET",
  path: "/res320/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r321 = route({
  id: "res321.get",
  method: "GET",
  path: "/res321/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r322 = route({
  id: "res322.get",
  method: "GET",
  path: "/res322/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r323 = route({
  id: "res323.get",
  method: "GET",
  path: "/res323/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r324 = route({
  id: "res324.get",
  method: "GET",
  path: "/res324/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r325 = route({
  id: "res325.get",
  method: "GET",
  path: "/res325/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r326 = route({
  id: "res326.get",
  method: "GET",
  path: "/res326/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r327 = route({
  id: "res327.get",
  method: "GET",
  path: "/res327/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r328 = route({
  id: "res328.get",
  method: "GET",
  path: "/res328/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r329 = route({
  id: "res329.get",
  method: "GET",
  path: "/res329/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r330 = route({
  id: "res330.get",
  method: "GET",
  path: "/res330/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r331 = route({
  id: "res331.get",
  method: "GET",
  path: "/res331/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r332 = route({
  id: "res332.get",
  method: "GET",
  path: "/res332/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r333 = route({
  id: "res333.get",
  method: "GET",
  path: "/res333/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r334 = route({
  id: "res334.get",
  method: "GET",
  path: "/res334/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r335 = route({
  id: "res335.get",
  method: "GET",
  path: "/res335/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r336 = route({
  id: "res336.get",
  method: "GET",
  path: "/res336/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r337 = route({
  id: "res337.get",
  method: "GET",
  path: "/res337/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r338 = route({
  id: "res338.get",
  method: "GET",
  path: "/res338/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r339 = route({
  id: "res339.get",
  method: "GET",
  path: "/res339/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r340 = route({
  id: "res340.get",
  method: "GET",
  path: "/res340/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r341 = route({
  id: "res341.get",
  method: "GET",
  path: "/res341/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r342 = route({
  id: "res342.get",
  method: "GET",
  path: "/res342/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r343 = route({
  id: "res343.get",
  method: "GET",
  path: "/res343/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r344 = route({
  id: "res344.get",
  method: "GET",
  path: "/res344/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r345 = route({
  id: "res345.get",
  method: "GET",
  path: "/res345/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r346 = route({
  id: "res346.get",
  method: "GET",
  path: "/res346/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r347 = route({
  id: "res347.get",
  method: "GET",
  path: "/res347/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r348 = route({
  id: "res348.get",
  method: "GET",
  path: "/res348/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r349 = route({
  id: "res349.get",
  method: "GET",
  path: "/res349/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r350 = route({
  id: "res350.get",
  method: "GET",
  path: "/res350/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r351 = route({
  id: "res351.get",
  method: "GET",
  path: "/res351/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r352 = route({
  id: "res352.get",
  method: "GET",
  path: "/res352/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r353 = route({
  id: "res353.get",
  method: "GET",
  path: "/res353/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r354 = route({
  id: "res354.get",
  method: "GET",
  path: "/res354/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r355 = route({
  id: "res355.get",
  method: "GET",
  path: "/res355/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r356 = route({
  id: "res356.get",
  method: "GET",
  path: "/res356/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r357 = route({
  id: "res357.get",
  method: "GET",
  path: "/res357/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r358 = route({
  id: "res358.get",
  method: "GET",
  path: "/res358/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r359 = route({
  id: "res359.get",
  method: "GET",
  path: "/res359/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r360 = route({
  id: "res360.get",
  method: "GET",
  path: "/res360/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r361 = route({
  id: "res361.get",
  method: "GET",
  path: "/res361/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r362 = route({
  id: "res362.get",
  method: "GET",
  path: "/res362/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r363 = route({
  id: "res363.get",
  method: "GET",
  path: "/res363/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r364 = route({
  id: "res364.get",
  method: "GET",
  path: "/res364/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r365 = route({
  id: "res365.get",
  method: "GET",
  path: "/res365/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r366 = route({
  id: "res366.get",
  method: "GET",
  path: "/res366/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r367 = route({
  id: "res367.get",
  method: "GET",
  path: "/res367/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r368 = route({
  id: "res368.get",
  method: "GET",
  path: "/res368/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r369 = route({
  id: "res369.get",
  method: "GET",
  path: "/res369/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r370 = route({
  id: "res370.get",
  method: "GET",
  path: "/res370/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r371 = route({
  id: "res371.get",
  method: "GET",
  path: "/res371/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r372 = route({
  id: "res372.get",
  method: "GET",
  path: "/res372/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r373 = route({
  id: "res373.get",
  method: "GET",
  path: "/res373/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r374 = route({
  id: "res374.get",
  method: "GET",
  path: "/res374/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r375 = route({
  id: "res375.get",
  method: "GET",
  path: "/res375/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r376 = route({
  id: "res376.get",
  method: "GET",
  path: "/res376/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r377 = route({
  id: "res377.get",
  method: "GET",
  path: "/res377/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r378 = route({
  id: "res378.get",
  method: "GET",
  path: "/res378/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r379 = route({
  id: "res379.get",
  method: "GET",
  path: "/res379/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r380 = route({
  id: "res380.get",
  method: "GET",
  path: "/res380/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r381 = route({
  id: "res381.get",
  method: "GET",
  path: "/res381/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r382 = route({
  id: "res382.get",
  method: "GET",
  path: "/res382/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r383 = route({
  id: "res383.get",
  method: "GET",
  path: "/res383/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r384 = route({
  id: "res384.get",
  method: "GET",
  path: "/res384/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r385 = route({
  id: "res385.get",
  method: "GET",
  path: "/res385/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r386 = route({
  id: "res386.get",
  method: "GET",
  path: "/res386/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r387 = route({
  id: "res387.get",
  method: "GET",
  path: "/res387/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r388 = route({
  id: "res388.get",
  method: "GET",
  path: "/res388/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r389 = route({
  id: "res389.get",
  method: "GET",
  path: "/res389/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r390 = route({
  id: "res390.get",
  method: "GET",
  path: "/res390/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r391 = route({
  id: "res391.get",
  method: "GET",
  path: "/res391/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r392 = route({
  id: "res392.get",
  method: "GET",
  path: "/res392/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r393 = route({
  id: "res393.get",
  method: "GET",
  path: "/res393/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r394 = route({
  id: "res394.get",
  method: "GET",
  path: "/res394/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r395 = route({
  id: "res395.get",
  method: "GET",
  path: "/res395/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r396 = route({
  id: "res396.get",
  method: "GET",
  path: "/res396/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r397 = route({
  id: "res397.get",
  method: "GET",
  path: "/res397/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r398 = route({
  id: "res398.get",
  method: "GET",
  path: "/res398/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r399 = route({
  id: "res399.get",
  method: "GET",
  path: "/res399/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r400 = route({
  id: "res400.get",
  method: "GET",
  path: "/res400/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r401 = route({
  id: "res401.get",
  method: "GET",
  path: "/res401/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r402 = route({
  id: "res402.get",
  method: "GET",
  path: "/res402/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r403 = route({
  id: "res403.get",
  method: "GET",
  path: "/res403/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r404 = route({
  id: "res404.get",
  method: "GET",
  path: "/res404/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r405 = route({
  id: "res405.get",
  method: "GET",
  path: "/res405/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r406 = route({
  id: "res406.get",
  method: "GET",
  path: "/res406/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r407 = route({
  id: "res407.get",
  method: "GET",
  path: "/res407/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r408 = route({
  id: "res408.get",
  method: "GET",
  path: "/res408/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r409 = route({
  id: "res409.get",
  method: "GET",
  path: "/res409/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r410 = route({
  id: "res410.get",
  method: "GET",
  path: "/res410/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r411 = route({
  id: "res411.get",
  method: "GET",
  path: "/res411/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r412 = route({
  id: "res412.get",
  method: "GET",
  path: "/res412/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r413 = route({
  id: "res413.get",
  method: "GET",
  path: "/res413/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r414 = route({
  id: "res414.get",
  method: "GET",
  path: "/res414/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r415 = route({
  id: "res415.get",
  method: "GET",
  path: "/res415/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r416 = route({
  id: "res416.get",
  method: "GET",
  path: "/res416/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r417 = route({
  id: "res417.get",
  method: "GET",
  path: "/res417/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r418 = route({
  id: "res418.get",
  method: "GET",
  path: "/res418/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r419 = route({
  id: "res419.get",
  method: "GET",
  path: "/res419/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r420 = route({
  id: "res420.get",
  method: "GET",
  path: "/res420/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r421 = route({
  id: "res421.get",
  method: "GET",
  path: "/res421/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r422 = route({
  id: "res422.get",
  method: "GET",
  path: "/res422/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r423 = route({
  id: "res423.get",
  method: "GET",
  path: "/res423/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r424 = route({
  id: "res424.get",
  method: "GET",
  path: "/res424/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r425 = route({
  id: "res425.get",
  method: "GET",
  path: "/res425/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r426 = route({
  id: "res426.get",
  method: "GET",
  path: "/res426/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r427 = route({
  id: "res427.get",
  method: "GET",
  path: "/res427/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r428 = route({
  id: "res428.get",
  method: "GET",
  path: "/res428/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r429 = route({
  id: "res429.get",
  method: "GET",
  path: "/res429/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r430 = route({
  id: "res430.get",
  method: "GET",
  path: "/res430/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r431 = route({
  id: "res431.get",
  method: "GET",
  path: "/res431/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r432 = route({
  id: "res432.get",
  method: "GET",
  path: "/res432/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r433 = route({
  id: "res433.get",
  method: "GET",
  path: "/res433/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r434 = route({
  id: "res434.get",
  method: "GET",
  path: "/res434/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r435 = route({
  id: "res435.get",
  method: "GET",
  path: "/res435/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r436 = route({
  id: "res436.get",
  method: "GET",
  path: "/res436/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r437 = route({
  id: "res437.get",
  method: "GET",
  path: "/res437/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r438 = route({
  id: "res438.get",
  method: "GET",
  path: "/res438/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r439 = route({
  id: "res439.get",
  method: "GET",
  path: "/res439/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r440 = route({
  id: "res440.get",
  method: "GET",
  path: "/res440/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r441 = route({
  id: "res441.get",
  method: "GET",
  path: "/res441/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r442 = route({
  id: "res442.get",
  method: "GET",
  path: "/res442/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r443 = route({
  id: "res443.get",
  method: "GET",
  path: "/res443/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r444 = route({
  id: "res444.get",
  method: "GET",
  path: "/res444/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r445 = route({
  id: "res445.get",
  method: "GET",
  path: "/res445/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r446 = route({
  id: "res446.get",
  method: "GET",
  path: "/res446/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r447 = route({
  id: "res447.get",
  method: "GET",
  path: "/res447/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r448 = route({
  id: "res448.get",
  method: "GET",
  path: "/res448/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r449 = route({
  id: "res449.get",
  method: "GET",
  path: "/res449/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r450 = route({
  id: "res450.get",
  method: "GET",
  path: "/res450/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r451 = route({
  id: "res451.get",
  method: "GET",
  path: "/res451/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r452 = route({
  id: "res452.get",
  method: "GET",
  path: "/res452/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r453 = route({
  id: "res453.get",
  method: "GET",
  path: "/res453/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r454 = route({
  id: "res454.get",
  method: "GET",
  path: "/res454/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r455 = route({
  id: "res455.get",
  method: "GET",
  path: "/res455/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r456 = route({
  id: "res456.get",
  method: "GET",
  path: "/res456/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r457 = route({
  id: "res457.get",
  method: "GET",
  path: "/res457/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r458 = route({
  id: "res458.get",
  method: "GET",
  path: "/res458/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r459 = route({
  id: "res459.get",
  method: "GET",
  path: "/res459/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r460 = route({
  id: "res460.get",
  method: "GET",
  path: "/res460/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r461 = route({
  id: "res461.get",
  method: "GET",
  path: "/res461/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r462 = route({
  id: "res462.get",
  method: "GET",
  path: "/res462/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r463 = route({
  id: "res463.get",
  method: "GET",
  path: "/res463/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r464 = route({
  id: "res464.get",
  method: "GET",
  path: "/res464/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r465 = route({
  id: "res465.get",
  method: "GET",
  path: "/res465/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r466 = route({
  id: "res466.get",
  method: "GET",
  path: "/res466/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r467 = route({
  id: "res467.get",
  method: "GET",
  path: "/res467/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r468 = route({
  id: "res468.get",
  method: "GET",
  path: "/res468/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r469 = route({
  id: "res469.get",
  method: "GET",
  path: "/res469/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r470 = route({
  id: "res470.get",
  method: "GET",
  path: "/res470/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r471 = route({
  id: "res471.get",
  method: "GET",
  path: "/res471/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r472 = route({
  id: "res472.get",
  method: "GET",
  path: "/res472/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r473 = route({
  id: "res473.get",
  method: "GET",
  path: "/res473/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r474 = route({
  id: "res474.get",
  method: "GET",
  path: "/res474/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r475 = route({
  id: "res475.get",
  method: "GET",
  path: "/res475/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r476 = route({
  id: "res476.get",
  method: "GET",
  path: "/res476/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r477 = route({
  id: "res477.get",
  method: "GET",
  path: "/res477/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r478 = route({
  id: "res478.get",
  method: "GET",
  path: "/res478/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r479 = route({
  id: "res479.get",
  method: "GET",
  path: "/res479/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r480 = route({
  id: "res480.get",
  method: "GET",
  path: "/res480/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r481 = route({
  id: "res481.get",
  method: "GET",
  path: "/res481/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r482 = route({
  id: "res482.get",
  method: "GET",
  path: "/res482/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r483 = route({
  id: "res483.get",
  method: "GET",
  path: "/res483/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r484 = route({
  id: "res484.get",
  method: "GET",
  path: "/res484/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r485 = route({
  id: "res485.get",
  method: "GET",
  path: "/res485/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r486 = route({
  id: "res486.get",
  method: "GET",
  path: "/res486/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r487 = route({
  id: "res487.get",
  method: "GET",
  path: "/res487/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r488 = route({
  id: "res488.get",
  method: "GET",
  path: "/res488/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r489 = route({
  id: "res489.get",
  method: "GET",
  path: "/res489/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r490 = route({
  id: "res490.get",
  method: "GET",
  path: "/res490/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r491 = route({
  id: "res491.get",
  method: "GET",
  path: "/res491/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r492 = route({
  id: "res492.get",
  method: "GET",
  path: "/res492/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r493 = route({
  id: "res493.get",
  method: "GET",
  path: "/res493/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r494 = route({
  id: "res494.get",
  method: "GET",
  path: "/res494/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r495 = route({
  id: "res495.get",
  method: "GET",
  path: "/res495/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r496 = route({
  id: "res496.get",
  method: "GET",
  path: "/res496/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r497 = route({
  id: "res497.get",
  method: "GET",
  path: "/res497/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r498 = route({
  id: "res498.get",
  method: "GET",
  path: "/res498/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});
const r499 = route({
  id: "res499.get",
  method: "GET",
  path: "/res499/item/:id",
  params: s.object({ id: s.integer({ minimum: 1, maximum: 500 }) }),
  response: { 200: s.object({ id: s.integer(), n: s.integer() }) },
  handle: ({ params }) => ({ id: params.id, n: 500 }),
});

export const app = defineApp({ id: "scale-500", modules: [ defineModule({ id: "res", routes: [r0, r1, r2, r3, r4, r5, r6, r7, r8, r9, r10, r11, r12, r13, r14, r15, r16, r17, r18, r19, r20, r21, r22, r23, r24, r25, r26, r27, r28, r29, r30, r31, r32, r33, r34, r35, r36, r37, r38, r39, r40, r41, r42, r43, r44, r45, r46, r47, r48, r49, r50, r51, r52, r53, r54, r55, r56, r57, r58, r59, r60, r61, r62, r63, r64, r65, r66, r67, r68, r69, r70, r71, r72, r73, r74, r75, r76, r77, r78, r79, r80, r81, r82, r83, r84, r85, r86, r87, r88, r89, r90, r91, r92, r93, r94, r95, r96, r97, r98, r99, r100, r101, r102, r103, r104, r105, r106, r107, r108, r109, r110, r111, r112, r113, r114, r115, r116, r117, r118, r119, r120, r121, r122, r123, r124, r125, r126, r127, r128, r129, r130, r131, r132, r133, r134, r135, r136, r137, r138, r139, r140, r141, r142, r143, r144, r145, r146, r147, r148, r149, r150, r151, r152, r153, r154, r155, r156, r157, r158, r159, r160, r161, r162, r163, r164, r165, r166, r167, r168, r169, r170, r171, r172, r173, r174, r175, r176, r177, r178, r179, r180, r181, r182, r183, r184, r185, r186, r187, r188, r189, r190, r191, r192, r193, r194, r195, r196, r197, r198, r199, r200, r201, r202, r203, r204, r205, r206, r207, r208, r209, r210, r211, r212, r213, r214, r215, r216, r217, r218, r219, r220, r221, r222, r223, r224, r225, r226, r227, r228, r229, r230, r231, r232, r233, r234, r235, r236, r237, r238, r239, r240, r241, r242, r243, r244, r245, r246, r247, r248, r249, r250, r251, r252, r253, r254, r255, r256, r257, r258, r259, r260, r261, r262, r263, r264, r265, r266, r267, r268, r269, r270, r271, r272, r273, r274, r275, r276, r277, r278, r279, r280, r281, r282, r283, r284, r285, r286, r287, r288, r289, r290, r291, r292, r293, r294, r295, r296, r297, r298, r299, r300, r301, r302, r303, r304, r305, r306, r307, r308, r309, r310, r311, r312, r313, r314, r315, r316, r317, r318, r319, r320, r321, r322, r323, r324, r325, r326, r327, r328, r329, r330, r331, r332, r333, r334, r335, r336, r337, r338, r339, r340, r341, r342, r343, r344, r345, r346, r347, r348, r349, r350, r351, r352, r353, r354, r355, r356, r357, r358, r359, r360, r361, r362, r363, r364, r365, r366, r367, r368, r369, r370, r371, r372, r373, r374, r375, r376, r377, r378, r379, r380, r381, r382, r383, r384, r385, r386, r387, r388, r389, r390, r391, r392, r393, r394, r395, r396, r397, r398, r399, r400, r401, r402, r403, r404, r405, r406, r407, r408, r409, r410, r411, r412, r413, r414, r415, r416, r417, r418, r419, r420, r421, r422, r423, r424, r425, r426, r427, r428, r429, r430, r431, r432, r433, r434, r435, r436, r437, r438, r439, r440, r441, r442, r443, r444, r445, r446, r447, r448, r449, r450, r451, r452, r453, r454, r455, r456, r457, r458, r459, r460, r461, r462, r463, r464, r465, r466, r467, r468, r469, r470, r471, r472, r473, r474, r475, r476, r477, r478, r479, r480, r481, r482, r483, r484, r485, r486, r487, r488, r489, r490, r491, r492, r493, r494, r495, r496, r497, r498, r499] }) ] });
