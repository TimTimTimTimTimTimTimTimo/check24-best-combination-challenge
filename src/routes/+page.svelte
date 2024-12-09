<script lang="ts">
    import * as Command from "$lib/components/ui/command/index.js";
    import * as Popover from "$lib/components/ui/popover/index.js";
    import { Button } from "$lib/components/ui/button/index.js";
    import * as Card from "$lib/components/ui/card/index.js";
    import * as Carousel from "$lib/components/ui/carousel/index.js";
    import { Separator } from "$lib/components/ui/separator/index.js";
    import { tick } from "svelte";
    import X from "lucide-svelte/icons/x";

    import { SvelteSet } from "svelte/reactivity";

    import { invoke } from "@tauri-apps/api/core";
    import {
        packages,
        type Package,
        teams,
        type Team,
    } from "$lib/generated_types";
    import type { Combination, Game, Offer } from "$lib/types";

    let selectedTeamIds: Set<number> = $state(new SvelteSet());

    type FetchCombinationsResponse = {
        game_count: number;
        best_combination: Combination;
        single_combinations: Combination[];
    };

    let combiResponse: FetchCombinationsResponse | null = $state(null);
    let combiLoading = $state(false);

    async function fetchCombinations() {
        combiLoading = true;
        combiResponse = await invoke("fetch_combinations", {
            team_ids: Array.from(selectedTeamIds),
        });
        combiLoading = false;
    }

    function getPackagesForCombi(combi: Combination): Package[] {
        let packagesForCombi = [];
        for (let i = 0; i < combi.package_ids.length; i++) {
            packagesForCombi.push(packages[combi.package_ids[i]]);
        }
        return packagesForCombi;
    }

    let teamSelectInput = $state("");
    let selectableTeams = $derived.by(() => {
        if (teamSelectInput.length == 0) {
            return teams.slice(0, 100);
        }

        let matching = teams.filter((team) =>
            team.toLowerCase().includes(teamSelectInput.toLowerCase()),
        );

        return matching.slice(0, 12);
    });

    let teamSelectOpen = $state(false);
    let teamTriggerRef: HTMLButtonElement = $state(null!);
    function teamCloseAndFocusTrigger() {
        teamSelectOpen = false;
        tick().then(() => {
            teamTriggerRef.focus();
        });
    }
</script>

<main class="p-10">
    <form>
        <label for="teams">Gewählte Teams: </label>
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
                        Team hinzufügen
                    </Button>
                {/snippet}
            </Popover.Trigger>
            <Popover.Content class="w-[200px] p-0">
                <Command.Root shouldFilter={false}>
                    <Command.Input
                        bind:value={teamSelectInput}
                        placeholder="Team suchen..."
                    />
                    <Command.List>
                        <Command.Empty>Kein Team gefunden.</Command.Empty>
                        <Command.Group>
                            {#each selectableTeams as team}
                                <Command.Item
                                    value={team}
                                    onSelect={() => {
                                        selectedTeamIds.add(
                                            teams.findIndex((t) => t === team),
                                        );
                                        fetchCombinations();
                                        teamCloseAndFocusTrigger();
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
        <div class="h-2"></div>
        <div id="teams" class="flex-row space-x-1">
            {#each selectedTeamIds as id (id)}
                <Button
                    onclick={() => {
                        selectedTeamIds.delete(id);
                        fetchCombinations();
                    }}>{teams[id]} <X /></Button
                >
            {/each}
        </div>
    </form>
    <br />
    {#if combiResponse}
        {@const { game_count, best_combination, single_combinations } =
            combiResponse}
        <Carousel.Root class="mx-10" opts={{ slidesToScroll: "auto" }}>
            <Carousel.Content>
                {@render CombinationCard(
                    best_combination.package_ids.length > 1
                        ? "Beste Kombination!"
                        : "Bestes Paket!",
                    game_count,
                    best_combination,
                )}
                {#each single_combinations as combi, i (i)}
                    {@render CombinationCard(i.toString(), game_count, combi)}
                {/each}
            </Carousel.Content>
            <Carousel.Previous />
            <Carousel.Next />
        </Carousel.Root>
    {:else}
        <div>Keine Teams ausgewählt.</div>
    {/if}
</main>

{#snippet CombinationCard(name: string, game_count: number, combi: Combination)}
    <Carousel.Item class="md:basis-1/3 lg:basis-1/6 h-96">
        <Card.Root class="hover:shadow">
            <Card.Header class="h-32">
                <Card.Title
                    class="flex justify-center items-center text-center h-14"
                >
                    {name}
                </Card.Title>
                <div class="h-14">
                    {getPackagesForCombi(combi)
                        .map((p) => p.name)
                        .join(" + ")}
                </div>
            </Card.Header>
            <Card.Content>
                <Separator class="my-4" />
                <div>Von {game_count} Spielen:</div>
                <div>
                    {combi.total_coverage} Gesamt
                </div>
                <div>{combi.live_coverage} Live</div>
                <div>
                    {combi.highlights_coverage} Highlights
                </div>
                <Separator class="my-4" />
                <div>
                    {(combi.total_price * 12) / 100}€ pro Jahr
                </div>
            </Card.Content>
        </Card.Root>
    </Carousel.Item>
{/snippet}
