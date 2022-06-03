import Head from "next/head";
import Image from "next/image";

import Layout from "@/components/layout";

import { getAllPosts } from "@/lib/api";
import Post from "@/types/post";

import styles from "@/styles/Home.module.css";

type Props = {
  allPosts: Post[]
};

const Index = ({ allPosts }: Props) => {
  const heroPost = allPosts[0];
  const morePosts = allPosts.slice(1);

  return (
    <Layout>
      <Head>
        <title>geoffjay.github.io</title>
        <meta name="description" content="GitHub pages for geoffjay" />
        <link rel="icon" href="/favicon.ico" />
      </Head>

      <main className={styles.main}>
        <h1 className={styles.title}>Geoff Johnson</h1>
      </main>

      <footer className={styles.footer}>
        <a
          href="https://github.com/geoffjay"
          target="_blank"
          rel="noopener noreferrer"
        >
          Powered by{" "}
          <span className={styles.logo}>
            <Image src="/vercel.svg" alt="Vercel Logo" width={72} height={16} />
          </span>
        </a>
      </footer>
    </Layout>
  );
}

export default Index;

export const getStaticProps = async () => {
  const allPosts = getAllPosts([
    "title",
    "date",
    "slug",
    "author",
    "coverImage",
    "excerpt",
  ]);

  return {
    props: { allPosts },
  };
};
