import { useState } from "react";
import { toast } from "sonner";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Dialog, DialogContent, DialogHeader, DialogTitle, DialogTrigger } from "@/components/ui/dialog";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Skeleton } from "@/components/ui/skeleton";
import { useCourses, useCreateCourse, useDeleteCourse } from "./api";

export function CoursesPage() {
  const { data: courses, isLoading } = useCourses();
  const create = useCreateCourse();
  const remove = useDeleteCourse();
  const [open, setOpen] = useState(false);

  async function onSubmit(e: React.FormEvent<HTMLFormElement>) {
    e.preventDefault();
    const f = new FormData(e.currentTarget);
    try {
      await create.mutateAsync({
        name: String(f.get("name")),
        code: (f.get("code") as string) || null,
        teacher: (f.get("teacher") as string) || null,
        color: String(f.get("color")),
      });
      toast.success("Cours ajouté");
      setOpen(false);
    } catch (err: any) {
      toast.error(err?.message ?? "Erreur");
    }
  }

  return (
    <div className="space-y-6 p-6">
      <div className="flex items-center justify-between">
        <h1 className="text-2xl font-semibold">Mes cours</h1>
        <Dialog open={open} onOpenChange={setOpen}>
          <DialogTrigger><Button>Nouveau cours</Button></DialogTrigger>
          <DialogContent>
            <DialogHeader><DialogTitle>Ajouter un cours</DialogTitle></DialogHeader>
            <form onSubmit={onSubmit} className="space-y-4">
              <div className="space-y-2"><Label htmlFor="name">Nom</Label><Input id="name" name="name" required /></div>
              <div className="space-y-2"><Label htmlFor="code">Code</Label><Input id="code" name="code" placeholder="INF301" /></div>
              <div className="space-y-2"><Label htmlFor="teacher">Enseignant</Label><Input id="teacher" name="teacher" /></div>
              <div className="space-y-2"><Label htmlFor="color">Couleur</Label><Input id="color" name="color" type="color" defaultValue="#6366f1" className="h-10 w-20 p-1" /></div>
              <Button type="submit" disabled={create.isPending} className="w-full">Enregistrer</Button>
            </form>
          </DialogContent>
        </Dialog>
      </div>

      {isLoading ? (
        <div className="grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
          {Array.from({ length: 3 }).map((_, i) => <Skeleton key={i} className="h-28" />)}
        </div>
      ) : courses?.length === 0 ? (
        <p className="text-muted-foreground">Aucun cours pour l'instant.</p>
      ) : (
        <div className="grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
          {courses?.map((c) => (
            <Card key={c.id} className="border-l-4" style={{ borderLeftColor: c.color }}>
              <CardHeader className="flex flex-row items-start justify-between">
                <div>
                  <CardTitle>{c.name}</CardTitle>
                  {c.code && <p className="text-sm text-muted-foreground">{c.code}</p>}
                </div>
                <Button variant="ghost" size="sm" onClick={() => remove.mutate(c.id)}>✕</Button>
              </CardHeader>
              {c.teacher && <CardContent className="text-sm">{c.teacher}</CardContent>}
            </Card>
          ))}
        </div>
      )}
    </div>
  );
}
