import App from "./routes/app";
import { Login } from "./routes/auth/login";
import { Layout } from "./Layout";
import { createBrowserRouter } from "react-router";
import { RouterProvider } from "react-router";
import { Signup } from "./routes/auth/signup";
import { Chat } from "./routes/chat/chat";
import { NewChat } from "./routes/chat/new";
import { Settings } from "./routes/settings";
import { KeysSettings } from "./routes/settings/keys";
import { ThemesSettings } from "./routes/settings/themes";

const router = createBrowserRouter([
  {
    Component: Layout,
    children: [
      {
        path: "",
        Component: App,
        children: [
          {
            path: "",
            Component: NewChat
          },
          {
            path: "chat/:chatId",
            Component: Chat
          },
          {
            path: "settings",
            Component: Settings,
          },
          {
            path: "settings/keys",
            Component: KeysSettings
          },
          {
            path: "settings/themes",
            Component: ThemesSettings
          }
        ]
      },
      {
        path: "auth",
        children: [
          {
            path: "login",
            Component: Login,
          },
          {
            path: "signup",
            Component: Signup,
          },
        ],
      },
    ],
  },
]);

export function Root() {
  return <RouterProvider router={router} />;
}
