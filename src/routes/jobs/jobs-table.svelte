<script lang="ts">
  import { createRender, createTable, Render, Subscribe } from "svelte-headless-table";
  import { addPagination, addSortBy, addTableFilter, addSelectedRows } from 'svelte-headless-table/plugins'; 
  import JobsTableCheckbox from "./jobs-table-checkbox.svelte";
  import JobsTableActions from "./jobs-table-actions.svelte";
  import { readable } from "svelte/store";
  import * as Table from "$lib/components/ui/table";
  import { Button } from "$lib/components/ui/button";
  import { Input } from "$lib/components/ui/input";
  import ArrowUpDown from "lucide-svelte/icons/arrow-up-down";
  import Star from 'lucide-svelte/icons/star';
  import type { Job } from './models';
	import { jobs } from "./store";

  const table = createTable(jobs, {
    page: addPagination(),
    sort: addSortBy(),
    filter: addTableFilter({
      fn: ({ filterValue, value }) =>
        value.toLowerCase().includes(filterValue.toLowerCase()),
    }),
    select: addSelectedRows(),
  });

  const columns = table.createColumns([
    table.column({
      accessor: "id",
      header: (_, { pluginStates }) => {
        const { allPageRowsSelected } = pluginStates.select;
        return createRender(JobsTableCheckbox, {
          checked: allPageRowsSelected,
        });
      },
      cell: ({ row }, { pluginStates }) => {
        const { getRowState } = pluginStates.select;
        const { isSelected } = getRowState(row);
 
        return createRender(JobsTableCheckbox, {
          checked: isSelected,
        });
      },
      plugins: {
        sort: {
          disable: true,
        },
        filter: {
          exclude: true,
        },
      },
    }),
    table.column({
      accessor: "active",
      header: "Active",
      plugins: {
        sort: {
          disable: true,
        },
      },
    }),
    table.column({
      accessor: "favorite",
      header: "Favorite",
      plugins: {
        sort: {
          disable: true,
        },
        filter: {
          exclude: true,
        },
      },
    }),
    table.column({
      accessor: "name",
      header: "Name",
    }),
    table.column({
      accessor: ({ id }) => id,
      header: "",
      cell: ({ value }) => {
        return createRender(JobsTableActions, { id: value });
      },
      plugins: {
        sort: {
          disable: true,
        },
        filter: {
          exclude: true,
        },
      },
    }),
  ]);

  const { 
    headerRows, 
    pageRows, 
    tableAttrs, 
    tableBodyAttrs, 
    pluginStates,
    flatColumns, 
    rows } = table.createViewModel(columns);

  const { hasNextPage, hasPreviousPage, pageIndex } = pluginStates.page;
  const { filterValue } = pluginStates.filter;

</script>

<div>
  <div class="flex items-center py-4">
    <Input
      class="max-w-sm"
      placeholder="Filter jobs..."
      type="text"
      bind:value={$filterValue}
    />
  </div>
  <div class="rounded-md border">
    <Table.Root {...$tableAttrs}>
      <Table.Header>
        {#each $headerRows as headerRow}
          <Subscribe rowAttrs={headerRow.attrs()}>
            <Table.Row>
              {#each headerRow.cells as cell (cell.id)}
                <Subscribe attrs={cell.attrs()} let:attrs props={cell.props()} let:props>
                  <Table.Head {...attrs} class="[&:has([role=checkbox])]:pl-3">
                    {#if cell.id === "name"}
                    <Button variant="ghost" on:click={props.sort.toggle}>
                      <Render of={cell.render()} />
                        <ArrowUpDown class={"ml-2 h-4 w-4"} />
                      </Button>
                    {:else}
                      <Render of={cell.render()} />
                    {/if}
                  </Table.Head>
                </Subscribe>
              {/each}
            </Table.Row>
          </Subscribe>
        {/each}
      </Table.Header>
      <Table.Body {...$tableBodyAttrs}>
        {#each $pageRows as row (row.id)}
          <Subscribe rowAttrs={row.attrs()} let:rowAttrs>
            <Table.Row {...rowAttrs}>
              {#each row.cells as cell (cell.id)}
                <Subscribe attrs={cell.attrs()} let:attrs>
                  <Table.Cell {...attrs}>
                    {#if cell.id === "favorite"}
                      {#if cell.value === true}
                        <div class="text-center font-medium">
                          <Star fill="yellow" class="w-4 h-4" />
                        </div>
                      {:else}
                        <div class="text-center font-medium">
                          <Star color="#333" class="w-4 h-4" />
                        </div>
                      {/if}
                    {:else}
                      <Render of={cell.render()} />
                    {/if}
                  </Table.Cell>
                </Subscribe>
              {/each}
            </Table.Row>
          </Subscribe>
        {/each}
      </Table.Body>
    </Table.Root>
  </div>
  <div class="flex items-center justify-end space-x-4 py-4">
    <Button
      variant="outline"
      size="sm"
      on:click={() => ($pageIndex = $pageIndex - 1)}
      disabled={!$hasPreviousPage}>Previous</Button
    >
    <Button
      variant="outline"
      size="sm"
      disabled={!$hasNextPage}
      on:click={() => ($pageIndex = $pageIndex + 1)}>Next</Button
    >
  </div>
</div>