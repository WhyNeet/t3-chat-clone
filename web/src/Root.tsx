import App from "./routes/app";
import { Login } from "./routes/auth/login";
import { Layout } from "./Layout";
import { createBrowserRouter } from "react-router";
import { RouterProvider } from "react-router";

const router = createBrowserRouter([
  {
    Component: Layout,
    children: [
      {
        index: true,
        Component: App
      },
      {
        path: "auth",
        children: [
          {
            path: "login",
            Component: Login
          }
        ]
      }
    ]
  }
])

export function Root() {
  return <RouterProvider router={router} />
}
