import { serve } from "./deps.ts";



const PORT = 8080;

const s = serve({ port: PORT });

const body = new TextEncoder().encode("a Deno app deployed on plurid.app\n");

console.log(`Server started on port ${PORT}`);
for await (const req of s) {
    req.respond({ body });
}
