import { Key, Telescope } from "lucide-react";
import { Button } from "../../components/ui/button";
import { useNavigate } from "react-router";
import { useChatsStore } from "../../lib/state/chats";
import { createChat } from "../../lib/api/chats";
import { useServiceStore } from "../../lib/state/service";
import { sendAndSubscribe } from "../../lib/api/messages";
import { Fragment } from "react/jsx-runtime";

export function NewChat() {
  const navigate = useNavigate();
  const addChat = useChatsStore((state) => state.initializeChat);
  const models = useServiceStore(state => state.models);

  const sendPrompt = async (prompt: string) => {
    const chat = await createChat();
    addChat(chat, []);
    navigate(`/chat/${chat.id}`);
    sendAndSubscribe(
      chat.id,
      models![0],
      {
        message: prompt,
        model: models![0].identifier,
        reasoning: null,
        use_search: false,
      },
      () => { },
    );
  }

  return (
    <div className="h-full w-full flex items-center justify-center px-4 md:px-10">
      <div className="max-w-3xl w-full">
        <h1 className="text-2xl font-bold font-display">Hello there!</h1>
        <p className="text-pink-900 font-display mb-4">
          What will you do today?
        </p>
        <div className="flex gap-2 mb-4">
          <Button
            intent="secondary"
            rounded="circle"
            size="small"
            className="px-3 py-1.5"
          >
            <Telescope className="h-5 w-5" />
            Explore Features
          </Button>
          <Button
            intent="secondary"
            rounded="circle"
            size="small"
            className="px-3 py-1.5"
          >
            <Key className="h-5 w-5" />
            Bring your own key
          </Button>
        </div>
        <h2 className="text-sm font-medium font-display text-pink-900/40 mb-2">
          Quick Prompts
        </h2>
        <div className="text-[15px]">
          {[
            "What is AI?",
            'How many Rs does the word "strawberry" contain?',
            "How to build my own AI chat?",
            "What is an MoE AI model?"
          ].map((prompt) => (
            <Fragment key={prompt}>
              <button
                onClick={() => sendPrompt(prompt)}
                className="w-full font-display px-3 py-2 rounded-lg text-pink-900 hover:bg-pink-50 cursor-pointer text-left"
              >
                {prompt}
              </button>
              <hr className="last:hidden my-1 text-pink-900/10" />
            </Fragment>
          ))}
        </div>
      </div>
    </div>
  );
}
