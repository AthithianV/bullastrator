<script lang="ts">
    import * as Card from "ui/card";
    import { cn } from "shared/utils";

    let {
        title,
        description,
        class: className,
        children,
    } = $props<{
        title: string;
        description: string;
        class?: string;
        children: import("svelte").Snippet;
    }>();
</script>

<div class="flex min-h-screen items-center justify-center p-6">
    <Card.Root class={cn("w-full max-w-xl p-12 bg-background", className)}>
        <Card.Header>
            <Card.Title class="text-4xl text-center">{title}</Card.Title>
            <Card.Description class="text-md text-center"
                >{description}</Card.Description
            >
        </Card.Header>
        <Card.Content>
            {@render children()}
        </Card.Content>
    </Card.Root>
</div>
