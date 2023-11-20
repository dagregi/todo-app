<script lang="ts">
    import { invalidateAll } from "$app/navigation";
    import type { PageData } from "./$types";

    export let data: PageData;
    let todos = data.todos;

    type Todo = {
        id: BigInt;
        description: string;
        completed: boolean;
    };

    async function delete_todo(id: BigInt) {
        await fetch(`http://127.0.0.1:3000/todos/${id}`, { method: "DELETE" });
        invalidateAll();
    }

    async function update_todo(todo: Todo) {
        await fetch(
            `http://127.0.0.1:3000/todos/${todo.id}?description=${todo.description}&completed=${todo.completed}`
        );
        invalidateAll();
    }
</script>

<div class="container p-2 mx-auto">
    <h1 class="h1 text-center py-8">Todos</h1>
    <form class="mb-6 mt-2" method="post" action="http://127.0.0.1:3000/todos">
        <input
            class="input"
            name="description"
            type="text"
            placeholder="Things to be done"
        />
    </form>
    <ol class="list">
        {#each todos as todo}
            <li class="py-1">
                <input
                    type="checkbox"
                    class="checkbox p-3"
                    bind:checked={todo.completed}
                    on:change={() => update_todo(todo)}
                />
                <input
                    type="text"
                    class="input"
                    bind:value={todo.description}
                    placeholder="Things to be done"
                />
                <span class="flex flex-row">
                    <button class="btn" on:click={() => update_todo(todo)}>
                        Update
                    </button>
                    <button class="btn" on:click={() => delete_todo(todo.id)}>
                        Delete
                    </button>
                </span>
            </li>
        {/each}
    </ol>
</div>
