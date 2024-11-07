<script lang="ts">
    import * as Command from "$lib/components/ui/command/index.js";
    import * as Table from "$lib/components/ui/table/index.js";
    import * as Popover from "$lib/components/ui/popover/index.js";
    import { Button } from "$lib/components/ui/button/index.js";
    import { tick } from "svelte";

    import { invoke } from "@tauri-apps/api/core";
    import { Teams, type Team } from "$lib/enums";
    import type { Game } from "$lib/types";

    let teams = Teams;
    let teamSelectOpen = $state(false);
    let selectedTeam: Team | null = $state(null);
    let teamTriggerRef: HTMLButtonElement = $state(null!);

    let filtered_games_promise: Promise<Game[]> = $derived.by(async () => {
        if (selectedTeam == null) {
            return [];
        }
        return await invoke("find_games_by_team", { team: selectedTeam });
    });

    function closeAndFocusTrigger() {
        teamSelectOpen = false;
        tick().then(() => {
            teamTriggerRef.focus();
        });
    }
</script>

<main class="p-10">
    <form>
        <label for="team">Team: </label>
        <Popover.Root bind:open={teamSelectOpen}>
            <Popover.Trigger bind:ref={teamTriggerRef}>
                {#snippet child({ props })}
                    <Button
                        variant="outline"
                        class="w-[200px] justify-between"
                        {...props}
                        role="combobox"
                        aria-expanded={teamSelectOpen}
                    >
                        {selectedTeam || "Wähle ein Team aus..."}
                    </Button>
                {/snippet}
            </Popover.Trigger>
            <Popover.Content class="w-[200px] p-0">
                <Command.Root>
                    <Command.Input id="team" placeholder="Team suchen..." />
                    <Command.List>
                        <Command.Empty>Kein Team gefunden.</Command.Empty>
                        <Command.Group>
                            {#each teams as team}
                                <Command.Item
                                    value={team}
                                    onSelect={() => {
                                        selectedTeam = team;
                                        closeAndFocusTrigger();
                                    }}
                                >
                                    {team}
                                </Command.Item>
                            {/each}
                        </Command.Group>
                    </Command.List>
                </Command.Root>
            </Popover.Content>
        </Popover.Root>
    </form>
    <br />
    <Table.Root>
        <Table.Header>
            <Table.Row>
                <Table.Head>Spiel</Table.Head>
            </Table.Row>
        </Table.Header>
        <Table.Body>
            {#await filtered_games_promise then filtered_games}
                {#each filtered_games as game, i (i)}
                    <Table.Row>
                        <Table.Cell
                            >{game.team_home} vs {game.team_away}</Table.Cell
                        >
                    </Table.Row>
                {/each}
            {/await}
        </Table.Body>
    </Table.Root>
</main>
