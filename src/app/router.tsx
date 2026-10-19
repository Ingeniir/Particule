import { createHashRouter } from "react-router";
import { AppLayout } from "./app-layout";
import { CoursesPage } from "@/features/courses/courses-page";

const Placeholder = ({ title }: { title: string }) => <div className="p-6 text-2xl font-semibold">{title}</div>;

// HashRouter : aucun souci de rechargement de page en production dans la webview
export const router = createHashRouter([
  {
    element: <AppLayout />,
    children: [
      { path: "/", element: <Placeholder title="Tableau de bord" /> },
      { path: "/courses", element: <CoursesPage /> },
      { path: "/schedule", element: <Placeholder title="Emploi du temps" /> },
      { path: "/tasks", element: <Placeholder title="Tâches" /> },
      { path: "/pomodoro", element: <Placeholder title="Pomodoro" /> },
    ],
  },
]);
