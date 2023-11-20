import type { PageLoad } from "./$types";

export const load: PageLoad = async ({ fetch }) => {
    const getTodos = async () => {
        const response = await fetch("http://localhost:3000/todos");
        const data = await response.json();
        return data;
    }

    return { todos: getTodos() }
}
