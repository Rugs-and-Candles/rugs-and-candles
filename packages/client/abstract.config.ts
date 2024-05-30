import { defineConfig } from '@abstract-money/cli'
import { react, vanilla} from '@abstract-money/cli/plugins'

export default defineConfig({
  out: 'src/generated',
  contracts: [
    {
      name: "board", // name of adapter
      path: "../../contracts/board/schema/", // path to the schema of the adapter
      namespace: "rugspaceandcandles", // namespace
      version: "0.1.0",
      moduleType: "adapter",
    },
    {
      name: "controller", // name of adapter
      path: "../../contracts/controller/schema/", // path to the schema of the adapter
      namespace: "rugspaceandcandles", // namespace
      version: "0.1.0",
      moduleType: "adapter",
    }
  ],
  plugins: [react(),
    vanilla({
      enableAppstractAppFor: ['controller', 'board']
    })
  ],
});
