import { AppProps } from "next/app";
import "../styles/globals.css";

function GithubSite({ Component, pageProps }: AppProps) {
  return <Component {...pageProps} />;
}

export default GithubSite;
