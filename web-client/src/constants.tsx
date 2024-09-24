import { AudioPage } from "./pages/AudioPage";
import { DocumentsPage } from "./pages/DocumentsPage";
import { EditPage } from "./pages/EditPage";

import { HomePage } from "./pages/HomePage";

type Page = {
  name: string;
  component: React.ReactNode;
  path: string;
};

export const pages: Page[] = [
  {
    name: "Home",
    component: <HomePage />,
    path: "/",
  },
  {
    name: "Audio",
    component: <AudioPage />,
    path: "/audio",
  },
  {
    name: "Documents",
    component: <DocumentsPage />,
    path: "/documents",
  },
];
