import { me } from "../api/auth";
import { chats } from "../api/chats";
import { is, subscribeToStream } from "../api/completions";
import { listKeys } from "../api/keys";
import { listModels } from "../api/service";
import { useAuthStore } from "./auth";
import { useChatsStore } from "./chats";
import { useServiceStore } from "./service";

export async function init() {
  const { setModels, setKeys } = useServiceStore.getState();

  listKeys().then(keys => {
    setKeys(keys);
  });

  const models = await listModels();
  setModels(models);
  const { updateUser } = useAuthStore.getState();

  const { setInferenceError } = useServiceStore.getState();
  const {
    initializeChat,
    finishFetching,
    updatePendingMessage,
    clearPendingMessage,
    addChatMessages,
    initPendingMessage,
    finishWebSearch,
    updateChatName,
  } = useChatsStore.getState();
  useAuthStore.subscribe((store, prev) => {
    if (store.user && !prev.user) {
      chats({
        start: 0,
        take: Math.round((window.innerHeight * 1.5) / 50),
      }).then((chats) => {
        for (const chat of chats) {
          initializeChat(chat);
          const streamId = localStorage.getItem(`stream-${chat.id}`);
          if (streamId) {
            const isSearching =
              typeof localStorage.getItem(
                `streaming-message-${chat.id}-search`,
              ) === "string";
            const message =
              localStorage.getItem(`streaming-message-${chat.id}`) ?? "";
            const reasoning =
              localStorage.getItem(`streaming-message-reasoning-${chat.id}`) ??
              "";
            const model = localStorage.getItem(
              `streaming-message-${chat.id}-model`,
            );
            const provider = models.find(m => m.name === model)!.base_url;
            initPendingMessage(chat.id, model ?? "AI", isSearching);
            updatePendingMessage(chat.id, { content: message, reasoning });
            subscribeToStream(
              streamId,
              (delta) => {
                updatePendingMessage(chat.id, {
                  reasoning: delta.reasoning,
                  content: delta.reasoning ? null : delta.content,
                });
                localStorage.setItem(
                  `streaming-message-${chat.id}`,
                  (localStorage.getItem(`streaming-message-${chat.id}`) ?? "") +
                  delta.content,
                );
                localStorage.setItem(
                  `streaming-message-reasoning-${chat.id}`,
                  (localStorage.getItem(
                    `streaming-message-reasoning-${chat.id}`,
                  ) ?? "") + (delta.reasoning ?? ""),
                );
              },
              (control) => {
                if (is.webSearchPerformed(control.control)) {
                  localStorage.removeItem(
                    `streaming-message-${chat.id}-search`,
                  );
                  finishWebSearch(chat.id);
                } else if (is.chatNameUpdated(control.control)) {
                  updateChatName(chat.id, control.control.name);
                } else if (is.inferenceError(control.control)) {
                  if (provider === "https://https://openrouter.ai/api/v1/chat/completions") {
                    setInferenceError("openrouter", control.control.code);
                  }
                  localStorage.removeItem(`stream-${chat.id}`);
                  localStorage.removeItem(`streaming-message-${chat.id}`);
                  localStorage.removeItem(
                    `streaming-message-reasoning-${chat.id}`,
                  );
                  localStorage.removeItem(`streaming-message-${chat.id}-search`);
                  localStorage.removeItem(`streaming-message-${chat.id}-model`);
                  clearPendingMessage(chat.id);
                }
              },
              (message) => {
                localStorage.removeItem(`stream-${chat.id}`);
                localStorage.removeItem(`streaming-message-${chat.id}`);
                localStorage.removeItem(
                  `streaming-message-reasoning-${chat.id}`,
                );
                localStorage.removeItem(`streaming-message-${chat.id}-search`);
                localStorage.removeItem(`streaming-message-${chat.id}-model`);
                clearPendingMessage(chat.id);
                addChatMessages(chat.id, [message]);
              },
            );
          }
        }
        finishFetching();
      });
    }
  });

  me().then((user) => {
    updateUser(user);
  });
}
