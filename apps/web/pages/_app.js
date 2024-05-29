import { ChakraProvider, GlobalStyle } from '@chakra-ui/react';
import "../styles/global.css";
import "@interchain-ui/react/styles";

import { wallets } from "cosmos-kit";
import { ChainProvider } from "@cosmos-kit/react";
import { assets, chains } from "chain-registry";
import {
  Box,
  ThemeProvider,
  useColorModeValue,
  useTheme,
} from "@interchain-ui/react";

function MyApp({ Component, pageProps }) {
  const { themeClass } = useTheme();
  const signerOptions = {
    // signingStargate: () => {
    //   return getSigningCosmosClientOptions();
    // }
  };

  return (
    <ThemeProvider>
      <ChainProvider
        chains={chains}
        assetLists={assets}
        wallets={wallets}
        walletConnectOptions={{
          signClient: {
            projectId: "a8510432ebb71e6948cfd6cde54b70f7",
            relayUrl: "wss://relay.walletconnect.org",
            metadata: {
              name: "CosmosKit Template",
              description: "CosmosKit dapp template",
              url: "https://docs.cosmology.zone/cosmos-kit/",
              icons: [],
            },
          },
        }}
        // @ts-ignore
        signerOptions={signerOptions}
      >

        <ChakraProvider>
          <GlobalStyle
            styles={{
              body: {
                backgroundImage: "url('icons/board-background.png')",
                backgroundSize: 'cover',
                backgroundPosition: 'center',
                backgroundRepeat: 'no-repeat',
                height: '100%',
                width: '100%',
                margin: 0,
                padding: 0,
                overflow: 'hidden',
              },
            }}
          />
          <Component {...pageProps} />
        </ChakraProvider>
      </ChainProvider>
    </ThemeProvider>

  );
}

export default MyApp;
