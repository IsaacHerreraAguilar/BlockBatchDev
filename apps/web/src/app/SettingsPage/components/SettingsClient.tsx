'use client';

import { FC } from "react";
import SettingsContent from "./SettingsContent";
import ClientOnly from "./ClientOnly";

const SettingsClient: FC = () => {
  return (
    <ClientOnly>
      <SettingsContent />
    </ClientOnly>
  );
};

export default SettingsClient;
