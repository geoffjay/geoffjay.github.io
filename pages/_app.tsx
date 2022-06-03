import { AppProps } from "next/app";
import { ChakraProvider } from "@chakra-ui/react";

import "@/styles/globals.css";

function App({ Component, pageProps }: AppProps) {
  return (
    <ChakraProvider>
      <Component {...pageProps} />
    </ChakraProvider>
  );
}

export default App;
