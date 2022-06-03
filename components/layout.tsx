import { Grid, Stack } from "@chakra-ui/react";

import Alert from "./alert";

type Props = {
  preview?: boolean;
  children: React.ReactNode;
};

const Layout = ({ preview, children }: Props) => {
  return (
    <Grid minH="100vh">
      <Stack direction="column">
        <Alert preview={preview} />
        {children}
      </Stack>
    </Grid>
  );
};

export default Layout;
