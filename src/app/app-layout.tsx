import { NavLink, Outlet } from "react-router";
import { BookOpen, CalendarDays, CheckSquare, Timer, LayoutDashboard } from "lucide-react";
import {
  Sidebar, SidebarContent, SidebarGroup, SidebarGroupLabel, SidebarHeader,
  SidebarInset, SidebarMenu, SidebarMenuButton, SidebarMenuItem, SidebarProvider, SidebarTrigger,
} from "@/components/ui/sidebar";

const nav = [
  { to: "/", label: "Tableau de bord", icon: LayoutDashboard },
  { to: "/courses", label: "Cours", icon: BookOpen },
  { to: "/schedule", label: "Emploi du temps", icon: CalendarDays },
  { to: "/tasks", label: "Tâches", icon: CheckSquare },
  { to: "/pomodoro", label: "Pomodoro", icon: Timer },
];

export function AppLayout() {
  return (
    <SidebarProvider>
      <Sidebar>
        <SidebarHeader className="px-4 py-3 text-lg font-bold">StudyTrack</SidebarHeader>
        <SidebarContent>
          <SidebarGroup>
            <SidebarGroupLabel>Navigation</SidebarGroupLabel>
            <SidebarMenu>
              {nav.map(({ to, label, icon: Icon }) => (
                <SidebarMenuItem key={to}>
                  <NavLink to={to} end>
                    {({ isActive }) => (
                      <SidebarMenuButton isActive={isActive}>
                        <Icon /> <span>{label}</span>
                      </SidebarMenuButton>
                    )}
                  </NavLink>
                </SidebarMenuItem>
              ))}
            </SidebarMenu>
          </SidebarGroup>
        </SidebarContent>
      </Sidebar>
      <SidebarInset>
        <header className="flex h-12 items-center border-b px-3"><SidebarTrigger /></header>
        <main className="flex-1 overflow-auto"><Outlet /></main>
      </SidebarInset>
    </SidebarProvider>
  );
}
