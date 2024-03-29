<script lang="ts">
  import { createTable, Render, Subscribe, createRender } from "svelte-headless-table";
  import { addPagination, addSortBy, addTableFilter, addHiddenColumns, addSelectedRows } from "svelte-headless-table/plugins";
  import { readable } from "svelte/store";
  import * as Table from "$lib/components/ui/table";
  import * as DropdownMenu from "$lib/components/ui/dropdown-menu";
  import DataTableActions from "./vendor-table-actions.svelte";
  import DataTableCheckbox from "./vendor-table-checkbox.svelte";
  import ArrowUpDown from "lucide-svelte/icons/arrow-up-down";
  import ChevronDown from "lucide-svelte/icons/chevron-down";
  import { Button } from "$lib/components/ui/button";
  import { Input } from "$lib/components/ui/input";

  import type { Vendor } from "./+page";

  export let vendors: Vendor[] = [];
  
  const table = createTable(readable(vendors), {
    page: addPagination(),
    sort: addSortBy({ disableMultiSort: true }),
    filter: addTableFilter({
      fn: ({ filterValue, value }) =>
        value.toLowerCase().includes(filterValue.toLowerCase()),
    }),
    hide: addHiddenColumns(),
    select: addSelectedRows(),
  });

  const columns = table.createColumns([
    table.column({
      accessor: "id",
      header: (_, { pluginStates }) => {
        const { allPageRowsSelected } = pluginStates.select;
        return createRender(DataTableCheckbox, {
          checked: allPageRowsSelected,
        });
      },
      cell: ({ row }, { pluginStates }) => {
        const { getRowState } = pluginStates.select;
        const { isSelected } = getRowState(row);
 
        return createRender(DataTableCheckbox, {
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
      accessor: "name",
      header: "Name",
      // plugins: {
      //   sort: {
      //     disable: true,
      //   },
      //   filter: {
      //     exclude: true,
      //   },
      // },
    }),
    table.column({
      accessor: "contact",
      header: "Contact",
    }),
    table.column({
      accessor: "email",
      header: "Email",
    }),
    table.column({
      accessor: "phone",
      header: "Phone",
    }),
    // table.column({
    //   accessor: "amount",
    //   header: "Amount",
    //   cell: ({ value }) => {
    //     const formatted = new Intl.NumberFormat("en-US", {
    //       style: "currency",
    //       currency: "USD",
    //     }).format(value);
    //     return formatted;
    //   },
    //   plugins: {
    //     sort: {
    //       disable: true,
    //     },
    //     filter: {
    //       exclude: true,
    //     },
    //   },
    // }),
    table.column({
      accessor: ({ id }) => id,
      header: "",
      cell: ({ value }) => {
        return createRender(DataTableActions, { id: value });
      },      
      plugins: {
        sort: {
          disable: true,
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
  const { hiddenColumnIds } = pluginStates.hide;
  const { selectedDataIds } = pluginStates.select;

  const ids = flatColumns.map((col) => col.id);
  let hideForId = Object.fromEntries(ids.map((id) => [id, true]));  

  $: $hiddenColumnIds = Object.entries(hideForId)
    .filter(([, hide]) => !hide)
    .map(([id]) => id);
 
  const hidableCols = ["email", "phone"];

</script>


<div>
  <div class="flex items-center py-4">
    <Input
      class="max-w-sm"
      placeholder="Filter emails..."
      type="text"
      bind:value={$filterValue}
    />
    <DropdownMenu.Root>
      <DropdownMenu.Trigger asChild let:builder>
        <Button variant="outline" class="ml-auto" builders={[builder]}>
          Columns <ChevronDown class="ml-2 h-4 w-4" />
        </Button>
      </DropdownMenu.Trigger>
      <DropdownMenu.Content>
        {#each flatColumns as col}
          {#if hidableCols.includes(col.id)}
            <DropdownMenu.CheckboxItem bind:checked={hideForId[col.id]}>
              {col.header}
            </DropdownMenu.CheckboxItem>
          {/if}
        {/each}
      </DropdownMenu.Content>
    </DropdownMenu.Root>    
  </div>
  <div class="rounded-md border">
    <Table.Root {...$tableAttrs}>
      <Table.Header>
        {#each $headerRows as headerRow}
          <Subscribe rowAttrs={headerRow.attrs()}>
            <Table.Row>
              {#each headerRow.cells as cell (cell.id)}
                <Subscribe 
                  attrs={cell.attrs()} 
                  let:attrs 
                  props={cell.props()}
                  let:props
                  >
                  <Table.Head {...attrs} class="[&:has([role=checkbox])]:pl-3">
                    <Render of={cell.render()} />
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
            <Table.Row 
              {...rowAttrs}
              data-state={$selectedDataIds[row.id] && "selected"}
              >
              {#each row.cells as cell (cell.id)}
                <Subscribe attrs={cell.attrs()} let:attrs>
                  <Table.Cell {...attrs}>
                    <Render of={cell.render()} />
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
    <div class="flex-1 text-sm text-muted-foreground">
      {Object.keys($selectedDataIds).length} of{" "}
      {$rows.length} row(s) selected.
    </div>
    <Button
      variant="outline"
      size="sm"
      on:click={() => ($pageIndex = $pageIndex - 1)}
      disabled={!$hasPreviousPage}>Previous</Button>
    <Button
      variant="outline"
      size="sm"
      disabled={!$hasNextPage}
      on:click={() => ($pageIndex = $pageIndex + 1)}>Next</Button>
  </div>
</div>