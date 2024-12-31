<script lang="ts">
    import * as Command from "$lib/components/ui/command/index.js";
    import * as Popover from "$lib/components/ui/popover/index.js";
    import { Button, buttonVariants } from "$lib/components/ui/button/index.js";
    import * as Card from "$lib/components/ui/card/index.js";
    import * as Carousel from "$lib/components/ui/carousel/index.js";
    import { Root, Separator } from "$lib/components/ui/separator/index.js";
    import * as Tooltip from "$lib/components/ui/tooltip/index.js";

    import { SvelteSet } from "svelte/reactivity";

    import { invoke } from "@tauri-apps/api/core";
    import {
        packages,
        type Package,
        teams,
        type Team,
        tournaments,
    } from "$lib/generated_types";
    import type {
        Combination,
        CombinationsResult,
        CombinationsQuery,
        Game,
        Offer,
    } from "$lib/types";
    import { fade } from "svelte/transition";
    import { Temporal } from "@js-temporal/polyfill";
    import { ModeWatcher, toggleMode } from "mode-watcher";
    import X from "lucide-svelte/icons/x";

    let selectedTeams: Set<Team> = $state(new SvelteSet());

    let combiResult: CombinationsResult | null = $state(null);
    let combiLoading = $state(false);

    async function fetchCombinations() {
        combiLoading = true;
        const start = performance.now();

        const query: CombinationsQuery = {
            team_ids: Array.from(selectedTeams).map((team) =>
                teams.findIndex((t) => t === team),
            ),
            tournament_ids: Array.from(
                { length: tournaments.length },
                (_, i) => i,
            ),
            timespan: {
                start: Temporal.PlainDate.from("1970-01-01"),
                end: Temporal.PlainDate.from("2050-12-31"),
            },
            cover_type: "Some",
        };
        combiResult = await invoke("fetch_combinations_handler", { query });
        const end = performance.now();
        console.log(`Query and calculations done in: ${end - start} ms`);

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
            return teams
                .filter((team) => !selectedTeams.has(team))
                .slice(0, 100);
        }

        let matching = teams.filter(
            (team) =>
                !selectedTeams.has(team) &&
                team.toLowerCase().includes(teamSelectInput.toLowerCase()),
        );

        return matching.slice(0, 12);
    });
</script>

<ModeWatcher />

<main class="p-10">
    <form class="flex">
        <Card.Root>
            <Card.Header class="flex flex-row">
                <Card.Title class="flex-grow">Teams</Card.Title>
                <Button
                    variant="outline"
                    class="mx-1"
                    onclick={() => {
                        selectedTeams = new Set();
                        fetchCombinations();
                    }}>Alle abwählen</Button
                >
                <Button
                    variant="secondary"
                    class="mx-1"
                    onclick={() => {
                        selectedTeams = new Set(teams);
                        fetchCombinations();
                    }}>Alle auswählen</Button
                >
            </Card.Header>

            <Card.Content class="flex flex-row h-80">
                <Command.Root class="w-48" shouldFilter={false}>
                    <Command.Input
                        bind:value={teamSelectInput}
                        placeholder="Team suchen..."
                    />
                    <Command.List class="h-56">
                        <Command.Empty>Kein Team gefunden.</Command.Empty>
                        {#if teamSelectInput.length === 0}
                            <Command.Group heading="Populäre Teams"
                            ></Command.Group>
                            {#each selectableTeams as team}
                                <Command.Item
                                    value={team}
                                    onSelect={() => {
                                        selectedTeams.add(team);
                                        fetchCombinations();
                                    }}
                                >
                                    {team}
                                </Command.Item>
                            {/each}
                        {/if}
                        <Command.Group>
                            {#each selectableTeams as team}
                                <Command.Item
                                    value={team}
                                    onSelect={() => {
                                        selectedTeams.add(team);
                                        fetchCombinations();
                                    }}
                                >
                                    {team}
                                </Command.Item>
                            {/each}
                        </Command.Group>
                    </Command.List>
                </Command.Root>
                <Separator orientation="vertical" class="mx-5" />
                <div class="flex flex-col w-72">
                    <h3 class="mb-2">
                        {selectedTeams.size} gewählte{selectedTeams.size === 1
                            ? "s"
                            : ""} Teams
                    </h3>
                    <ul class="overflow-y-scroll">
                        {#each selectedTeams as team}
                            <li>
                                <Button
                                    variant="secondary"
                                    class="my-1 max-w-72"
                                    onclick={() => selectedTeams.delete(team)}
                                >
                                    {team}
                                </Button>
                            </li>
                        {/each}
                    </ul>
                </div></Card.Content
            >
        </Card.Root>
        <Button onclick={toggleMode}>Nachtmodus an/aus</Button>
    </form>
    <br />
    <div class="contents" in:fade>
        <Carousel.Root class="mx-10 px-2" opts={{ slidesToScroll: "auto" }}>
            <Carousel.Content class="mx-2">
                {#if combiResult}
                    {@const {
                        game_count,
                        orphan_count,
                        cheapest_combination,
                        smallest_combination,
                        single_combinations,
                    } = combiResult}
                    {@render CombinationCard(
                        cheapest_combination.package_ids.length > 1
                            ? "Beste Kombination!"
                            : "Bestes Paket!",
                        game_count,
                        orphan_count,
                        cheapest_combination,
                    )}
                    {#if smallest_combination}
                        {@render CombinationCard(
                            smallest_combination.package_ids.length > 1
                                ? "Kleinste Kombination!"
                                : "Kleinstes Paket!",
                            game_count,
                            orphan_count,
                            smallest_combination,
                        )}
                    {/if}
                    {#each single_combinations as single}
                        {@render CombinationCard(
                            "",
                            game_count,
                            orphan_count,
                            single,
                        )}
                    {/each}
                {:else}
                    <div>Keine Teams ausgewählt.</div>
                {/if}
            </Carousel.Content>
            <Carousel.Previous />
            <Carousel.Next />
        </Carousel.Root>
    </div>
</main>

{#snippet CombinationCard(
    name: string,
    game_count: number,
    orphan_count: number,
    combi: Combination,
)}
    <Carousel.Item class="md:basis-1/3 lg:basis-1/6 py-4">
        <Card.Root class="hover:shadow hover:scale-[1.01] transition-transform">
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
                {#if orphan_count > 0}
                    <div>
                        Für {orphan_count} Spiele gibt es kein Streaming Angebot!
                    </div>
                {/if}
                <div>Von {game_count} streambaren Spielen:</div>
                <div>
                    {combi.coverages.some_coverage} Gesamt
                </div>
                <div>{combi.coverages.live_coverage} Live</div>
                <div>
                    {combi.coverages.high_coverage} Highlights
                </div>
                <div>
                    {combi.coverages.full_coverage} Beides
                </div>
                <Separator class="my-4" />
                <div>
                    {combi.yearly_price_per_month_cents / 100}€ pro Monat im
                    Jahresabo
                </div>
            </Card.Content>
        </Card.Root>
    </Carousel.Item>
{/snippet}
