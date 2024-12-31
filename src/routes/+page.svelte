<script lang="ts">
    import * as Command from "$lib/components/ui/command/index.js";
    import { Button } from "$lib/components/ui/button/index.js";
    import * as Card from "$lib/components/ui/card/index.js";
    import * as Carousel from "$lib/components/ui/carousel/index.js";
    import { Separator } from "$lib/components/ui/separator/index.js";
    import { RangeCalendar } from "$lib/components/ui/range-calendar/index.js";
    import { SvelteSet } from "svelte/reactivity";
    import { CalendarDate, parseDate } from "@internationalized/date";
    import * as Select from "$lib/components/ui/select/index.js";
    import { ModeWatcher, toggleMode } from "mode-watcher";
    import { invoke } from "@tauri-apps/api/core";
    import {
        packages,
        type Package,
        teams,
        tournaments,
        type Game,
        type OrphanGame,
        games,
        orphan_games,
    } from "$lib/generated_types";
    import type {
        Combination,
        CombinationsResult,
        CombinationsQuery,
        CoverType,
    } from "$lib/types";
    import Sun from "lucide-svelte/icons/sun";
    import Moon from "lucide-svelte/icons/moon";

    let selectedGames: Game[] = $state(games);
    let selectedOrphanGames: OrphanGame[] = $state(orphan_games);
    let selectedTeamIds: Set<number> = $state(new SvelteSet());
    let selectedTournamentIds: Set<number> = $state(
        new SvelteSet(Array.from({ length: tournaments.length }, (_, i) => i)),
    );
    let selectedDateRange = $state({
        start: new CalendarDate(2000, 1, 1),
        end: new CalendarDate(3000, 12, 31),
    });
    let coverType: CoverType = $state("Some");
    let combiResult: CombinationsResult | null = $state(null);

    // Function to update selected games based on filters
    function updateSelectedGames() {
        let filteredGames: Game[] = [];
        let filteredOrphanGames: OrphanGame[] = [];

        for (let game of games) {
            if (
                (selectedTeamIds.has(game.attributes.team_away_id) ||
                    selectedTeamIds.has(game.attributes.team_home_id)) &&
                selectedTournamentIds.has(game.attributes.tournament_id) &&
                selectedDateRange.start < parseDate(game.attributes.date) &&
                parseDate(game.attributes.date) < selectedDateRange.end
            ) {
                filteredGames.push(game);
            }
        }

        for (let orphanGame of orphan_games) {
            if (
                (selectedTeamIds.has(orphanGame.attributes.team_away_id) ||
                    selectedTeamIds.has(orphanGame.attributes.team_home_id)) &&
                selectedTournamentIds.has(
                    orphanGame.attributes.tournament_id,
                ) &&
                selectedDateRange.start <
                    parseDate(orphanGame.attributes.date) &&
                parseDate(orphanGame.attributes.date) < selectedDateRange.end
            ) {
                filteredOrphanGames.push(orphanGame);
            }
        }

        console.log(filteredGames);
        selectedGames = filteredGames;
        selectedOrphanGames = filteredOrphanGames;
        fetchCombinations();
    }

    let game_count = $derived(selectedGames.length);
    let orphan_count = $derived(selectedOrphanGames.length);

    // Function to fetch combinations based on selected games and cover type
    async function fetchCombinations() {
        const start = performance.now();

        const query: CombinationsQuery = {
            game_ids: Array.from(selectedGames).map((game) => game.id),
            cover_type: coverType,
        };
        combiResult = await invoke("fetch_combinations_handler", { query });
        const end = performance.now();
        console.log(`Query and calculations done in: ${end - start} ms`);
    }

    // Function to get packages for a combination
    function getPackagesForCombi(combi: Combination): Package[] {
        return combi.package_ids.map((id) => packages[id]);
    }

    $inspect(selectedGames);
    $inspect(selectedDateRange);
</script>

<main class="p-10">
    <ModeWatcher />
    <Carousel.Root class="mx-5" opts={{ slidesToScroll: "auto" }}>
        <Carousel.Content>
            <Carousel.Item class="md:basis-full lg:basis-2/5">
                <Card.Root>
                    <Card.Header class="flex flex-row">
                        <Card.Title class="flex-grow">Teams</Card.Title>
                        <Button
                            variant="outline"
                            class="mx-1"
                            onclick={() => {
                                selectedTeamIds.clear();
                                updateSelectedGames();
                            }}>Alle abwählen</Button
                        >
                        <Button
                            variant="secondary"
                            class="mx-1"
                            onclick={() => {
                                for (let i = 0; i < teams.length; i++) {
                                    selectedTeamIds.add(i);
                                }
                                updateSelectedGames();
                            }}>Alle auswählen</Button
                        >
                    </Card.Header>

                    <Card.Content class="flex flex-row h-80">
                        <Command.Root class="w-48 flex-shrink-0">
                            <Command.Input placeholder="Team suchen..." />
                            <Command.List class="max-h-full">
                                <Command.Empty
                                    >Kein Team gefunden.</Command.Empty
                                >
                                {#each Array(teams.length)
                                    .fill(0)
                                    .map((_, i) => i) as teamId}
                                    {@const team = teams[teamId]}
                                    {#if !selectedTeamIds.has(teamId)}
                                        <Command.Item
                                            value={team}
                                            onSelect={() => {
                                                selectedTeamIds.add(teamId);
                                                updateSelectedGames();
                                            }}
                                        >
                                            {team}
                                        </Command.Item>
                                    {/if}
                                {/each}
                                <Command.Group heading="Gewählte Teams">
                                    {#each selectedTeamIds as teamId (teamId)}
                                        {@const team = teams[teamId]}
                                        <Command.Item
                                            value={team}
                                            onSelect={() => {
                                                selectedTeamIds.delete(teamId);
                                                updateSelectedGames();
                                            }}
                                        >
                                            {team}
                                        </Command.Item>
                                    {/each}
                                </Command.Group>
                            </Command.List>
                        </Command.Root>
                        <Separator orientation="vertical" class="mx-5" />
                        <div class="flex flex-col max-w-fit">
                            <h3 class="mb-2">
                                {selectedTeamIds.size} gewählte{selectedTeamIds.size ===
                                1
                                    ? "s"
                                    : ""} Teams
                            </h3>
                            <div
                                class="overflow-y-scroll overflow-x-clip flex flex-wrap max-w-full"
                            >
                                {#each selectedTeamIds as teamId (teamId)}
                                    {@const team = teams[teamId]}
                                    <Button
                                        variant="secondary"
                                        class="m-1 max-w-full whitespace-normal"
                                        onclick={() => {
                                            selectedTeamIds.delete(teamId);
                                            updateSelectedGames();
                                        }}
                                    >
                                        {team}
                                    </Button>
                                {/each}
                            </div>
                        </div>
                    </Card.Content>
                </Card.Root>
            </Carousel.Item>
            <Carousel.Item class="md:basis-full lg:basis-2/5">
                <Card.Root>
                    <Card.Header class="flex flex-row">
                        <Card.Title class="flex-grow">Turniere</Card.Title>
                        <Button
                            variant="outline"
                            class="mx-1"
                            onclick={() => {
                                selectedTournamentIds.clear();
                                updateSelectedGames();
                            }}>Alle abwählen</Button
                        >
                        <Button
                            variant="secondary"
                            class="mx-1"
                            onclick={() => {
                                for (let i = 0; i < tournaments.length; i++) {
                                    selectedTournamentIds.add(i);
                                }
                                updateSelectedGames();
                            }}>Alle auswählen</Button
                        >
                    </Card.Header>

                    <Card.Content class="flex flex-row h-80">
                        <Command.Root class="w-48 flex-shrink-0">
                            <Command.Input placeholder="Turnier suchen..." />
                            <Command.List class="max-h-full">
                                <Command.Empty
                                    >Kein Turnier gefunden.</Command.Empty
                                >
                                <Command.Group>
                                    {#each Array(tournaments.length)
                                        .fill(0)
                                        .map((_, i) => i) as tournamentId}
                                        {@const tournament =
                                            tournaments[tournamentId]}
                                        {#if !selectedTournamentIds.has(tournamentId)}
                                            <Command.Item
                                                value={tournament}
                                                onSelect={() => {
                                                    selectedTournamentIds.add(
                                                        tournamentId,
                                                    );
                                                    updateSelectedGames();
                                                }}
                                            >
                                                {tournament}
                                            </Command.Item>
                                        {/if}
                                    {/each}
                                </Command.Group>
                                <Command.Group heading="Gewählte Turniere">
                                    {#each selectedTournamentIds as tournamentId (tournamentId)}
                                        {@const tournament =
                                            tournaments[tournamentId]}
                                        <Command.Item
                                            value={tournament}
                                            onSelect={() => {
                                                selectedTournamentIds.delete(
                                                    tournamentId,
                                                );
                                                updateSelectedGames();
                                            }}
                                        >
                                            {tournament}
                                        </Command.Item>
                                    {/each}
                                </Command.Group>
                            </Command.List>
                        </Command.Root>
                        <Separator orientation="vertical" class="mx-5" />
                        <div class="flex flex-col max-w-fit">
                            <h3 class="mb-2">
                                {selectedTournamentIds.size} gewählte{selectedTournamentIds.size ===
                                1
                                    ? "s"
                                    : ""} Turniere
                            </h3>
                            <div
                                class="overflow-y-scroll overflow-x-clip flex flex-wrap max-w-full"
                            >
                                {#each selectedTournamentIds as tournamentId (tournamentId)}
                                    {@const tournament =
                                        tournaments[tournamentId]}
                                    <Button
                                        variant="secondary"
                                        class="m-1 max-w-full whitespace-normal"
                                        onclick={() => {
                                            selectedTournamentIds.delete(
                                                tournamentId,
                                            );
                                            updateSelectedGames();
                                        }}
                                    >
                                        {tournament}
                                    </Button>
                                {/each}
                            </div>
                        </div>
                    </Card.Content>
                </Card.Root>
            </Carousel.Item>
            <Carousel.Item class="md:basis-full lg:basis-1/5">
                <Card.Root>
                    <Card.Header class="flex flex-row">
                        <Card.Title class="flex-grow">Zeitraum</Card.Title>
                    </Card.Header>

                    <Card.Content
                        class="flex max-h-80 justify-center items-center p-10"
                    >
                        <RangeCalendar bind:value={selectedDateRange} />
                    </Card.Content>
                </Card.Root>
            </Carousel.Item>
        </Carousel.Content>
        <Carousel.Previous />
        <Carousel.Next />
    </Carousel.Root>
    <Separator class="my-4" />
    <div class="flex flex-row">
        <Button onclick={toggleMode} variant="outline" size="icon">
            <Sun
                class="h-[1.2rem] w-[1.2rem] rotate-0 scale-100 transition-all dark:-rotate-90 dark:scale-0"
            />
            <Moon
                class="absolute h-[1.2rem] w-[1.2rem] rotate-90 scale-0 transition-all dark:rotate-0 dark:scale-100"
            />
        </Button>
        <Select.Root
            type="single"
            bind:value={coverType}
            onValueChange={() => fetchCombinations()}
        >
            <Select.Trigger class="w-[180px]">{coverType}</Select.Trigger>
            <Select.Content>
                <Select.Item value="High">Highlights</Select.Item>
                <Select.Item value="Live">Live</Select.Item>
                <Select.Item value="Some">Eins von Beiden</Select.Item>
                <Select.Item value="Full">Beide</Select.Item>
            </Select.Content>
        </Select.Root>
    </div>
    <br />
    <Carousel.Root class="mx-5 px-2" opts={{ slidesToScroll: "auto" }}>
        <Carousel.Content class="mx-2">
            {#if combiResult}
                {@const {
                    cheapest_combination,
                    smallest_combination,
                    single_combinations,
                } = combiResult}
                {#if cheapest_combination.package_ids.length > 0}
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
                {:else}
                    <Carousel.Item class="md:basis-1/3 lg:basis-1/6 py-1">
                        <Card.Root
                            class="hover:shadow hover:scale-[1.01] transition-transform"
                        >
                            <Card.Header class="h-32">
                                <Card.Title
                                    class="flex justify-center items-center text-center h-14"
                                >
                                    Keine sinnvolle Kombination gefunden
                                </Card.Title>
                            </Card.Header>
                        </Card.Root>
                    </Carousel.Item>
                {/if}

                {#each single_combinations.toSorted((a, b) => {
                    switch (coverType) {
                        case "High":
                            return b.coverages.high_coverage - a.coverages.high_coverage;
                        case "Live":
                            return b.coverages.live_coverage - a.coverages.live_coverage;
                        case "Some":
                            return b.coverages.some_coverage - a.coverages.some_coverage;
                        case "Full":
                            return b.coverages.full_coverage - a.coverages.full_coverage;
                        default:
                            return 0;
                    }
                }) as single}
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
</main>

{#snippet CombinationCard(
    name: string,
    game_count: number,
    orphan_count: number,
    combi: Combination,
)}
    <Carousel.Item class="md:basis-1/3 lg:basis-1/6 py-1">
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
