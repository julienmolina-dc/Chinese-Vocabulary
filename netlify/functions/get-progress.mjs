import { getStore } from "@netlify/blobs";

export default async (req, context) => {
  const url = new URL(req.url);
  const userKey = url.searchParams.get("key");
  
  if (!userKey || userKey.length < 3) {
    return new Response(JSON.stringify({ error: "Missing or invalid key" }), { 
      status: 400,
      headers: { "Content-Type": "application/json", "Access-Control-Allow-Origin": "*" }
    });
  }

  const store = getStore("flashcard-progress");
  const data = await store.get(userKey, { type: "json" });
  
  return new Response(JSON.stringify(data || {}), {
    headers: { "Content-Type": "application/json", "Access-Control-Allow-Origin": "*" }
  });
};
