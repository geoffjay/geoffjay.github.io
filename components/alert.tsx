import { Flex, Link, Stack, Text } from "@chakra-ui/react";

type Props = {
  preview?: boolean;
};

const Alert = ({ preview }: Props) => {
  return (
    <Stack w="100%">
      {preview ? (
        <Flex>
          <Text>This page is a preview. </Text>
          <Link to="/api/exit-preview">Click here</Link>
          <Text> to exit preview mode.</Text>
        </Flex>
      ) : (
        <Flex>
          <Text>The code for this blog is </Text>
          <Link to="https://github.com/geoffjay/geoffjay.github.io">
            available on GitHub
          </Link>
        </Flex>
      )}
    </Stack>
  );
};

export default Alert;
