type Flags = {
   dryRun: boolean;
   verbose: boolean;
};

const defaultFlags: Flags = { dryRun: false, verbose: false };

function withVerbose(flags: Flags = defaultFlags): Flags {
   flags.verbose = true;
   return flags;
}
