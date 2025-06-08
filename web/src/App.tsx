import { useState } from "react";
import { isControl, type CompletionData } from "./lib/completion";
import Markdown from "react-markdown";

function App() {
  const [message, setMessage] = useState("");
  const [result, setResult] = useState("");

  const runPrompt = async () => {
    const res = await fetch("http://localhost:8080/completions/prompt", {
      method: "POST",
      headers: {
        Accept: "application/json",
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ message, model: "google/gemma-3-27b-it:free" }),
    });
    const { stream_id } = await res.json();

    const events = new EventSource(
      `http://localhost:8080/completions/prompt/sse/${stream_id}`,
    );

    events.addEventListener("message", (message: MessageEvent<string>) => {
      const data: CompletionData = JSON.parse(message.data);
      if (isControl(data)) {
        switch (data.control) {
          case "done":
            events.close();
            return;
        }
      }
      setResult((prev) => prev + data.content);
    });
  };

  return (
    <main className="h-screen w-screen p-2">
      {result.length ? (
        <div className="wrap-normal p-2 border border-black/10 prose max-w-full">
          <Markdown>{result}</Markdown>
        </div>
      ) : null}
      <div className="flex gap-2">
        <input
          value={message}
          onChange={(e) => setMessage(e.currentTarget.value)}
          type="text"
          className="px-2 py-1 border border-black/10 w-full"
        />
        <button onClick={runPrompt}>Send</button>
      </div>
    </main>
  );
}

export default App;
