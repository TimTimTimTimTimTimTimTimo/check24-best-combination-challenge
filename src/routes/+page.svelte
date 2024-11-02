<script lang="ts">
    import { Input } from "$lib/components/ui/input/index";
    import { Button } from "$lib/components/ui/button/index";
</script>

<main>
    <div class="p-8 flex justify-center items-center w-full space-x-2">
        <Input type="search" placeholder="Team" class="max-w-xs" />
        <Button type="submit">Finde alle Spiele</Button>
    </div>
</main>
