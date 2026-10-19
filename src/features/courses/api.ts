import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { commands, type NewCourse } from "@/lib/bindings";
import { unwrap } from "@/lib/ipc";

export const courseKeys = { all: ["courses"] as const};

export const useCourses = () => 
    useQuery({ queryKey: courseKeys.all, queryFn: () => unwrap(commands.listCourses()) });

export function useCreateCourse() {
    const qc = useQueryClient();

    return useMutation({
        mutationFn: (input: NewCourse) => unwrap(commands.createCourse(input)),
        onSuccess: () => qc.invalidateQueries({ queryKey: courseKeys.all }),
    });
}

export function useDeleteCourse() {
    const qc = useQueryClient();

    return useMutation({
        mutationFn: (id: string) => unwrap(commands.deleteCourse(id)),
        onSuccess: () => qc.invalidateQueries({ queryKey: courseKeys.all }),
    });
}