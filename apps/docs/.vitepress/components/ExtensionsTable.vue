<script setup lang="ts">
import { createColumnHelper } from "@tanstack/vue-table";
import { vTooltip as tooltip } from "floating-vue";
import { VPBadge } from "vitepress/theme";
import { h, withDirectives } from "vue";

import type { ExtensionFrontmatter } from "../types";
import BaseTable from "./BaseTable.vue";

type Row = {
  url: string;
  frontmatter: ExtensionFrontmatter;
};

const props = defineProps<{ data: Row[] }>();
const columnHelper = createColumnHelper<Row>();

const columns = [
  columnHelper.accessor("frontmatter.title", {
    header: "Extension",
    cell: (info) =>
      withDirectives(
        h(
          "a",
          {
            href: info.row.original.url,
          },
          info.getValue(),
        ),
        [[tooltip, info.row.original.frontmatter.description]],
      ),
  }),
  columnHelper.accessor("frontmatter.implemented", {
    header: "Implemented",
    cell: (info) =>
      info.getValue() ? h("div", { class: "i-heroicons-check" }) : null,
  }),
  columnHelper.accessor("frontmatter.default", {
    header: "Default",
    cell: (info) =>
      info.getValue() ? h("div", { class: "i-heroicons-check" }) : null,
  }),
  columnHelper.accessor("frontmatter.cloud_only", {
    header: "Cloud Only",
    cell: (info) =>
      info.getValue() ? h("div", { class: "i-heroicons-check" }) : null,
  }),
];
</script>

<template>
  <BaseTable :data="data" :columns="columns" />
</template>
