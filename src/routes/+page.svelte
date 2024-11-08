<script lang="ts">
    import * as Command from "$lib/components/ui/command/index.js";
    import * as Table from "$lib/components/ui/table/index.js";
    import * as Popover from "$lib/components/ui/popover/index.js";
    import { Button } from "$lib/components/ui/button/index.js";
    import { tick } from "svelte";

    import { invoke } from "@tauri-apps/api/core";
    import {
        packages,
        type Package,
        teams,
        type Team,
    } from "$lib/generated_types";
    import type { Game, Offer } from "$lib/types";

    type GamesAndOffers = {
        games: Game[];
        offers_map: { [key: number]: Offer[] };
    };

    let teamSelectOpen = $state(false);
    let selectedTeam: Team | null = $state(null);
    let teamTriggerRef: HTMLButtonElement = $state(null!);

    let filteredGamesAndOffersPromise: Promise<GamesAndOffers> = $derived.by(
        async () => {
            if (selectedTeam == null) {
                return { games: [], offers_map: {} };
            }
            return await invoke("find_games_and_offers_by_team", {
                team: selectedTeam,
            });
        },
    );

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
    {#await filteredGamesAndOffersPromise then { games, offers_map }}
        <Table.Root>
            <Table.Header>
                <Table.Row>
                    <Table.Head>Spiel</Table.Head>
                    {#each packages as pack, i (i)}
                        <Table.Head>{pack.name}</Table.Head>
                    {/each}
                </Table.Row>
            </Table.Header>
            <Table.Body>
                {#each games as game, i (i)}
                    <Table.Row>
                        <Table.Cell
                            >{game.team_home} vs {game.team_away}</Table.Cell
                        >
                        {#each packages as pack, i (i)}
                            <Table.Cell>
                                {@const offer = offers_map[game.id]?.find(
                                    (o) => o.package_id === pack.id,
                                )}
                                {#if offer}
                                    {#if offer.live}
                                        live
                                    {:else}
                                        hightlights
                                    {/if}
                                {/if}
                            </Table.Cell>
                        {/each}
                    </Table.Row>
                {/each}
            </Table.Body>
        </Table.Root>
    {/await}
</main>
