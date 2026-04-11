function shouldDeploy(isMainBranch: boolean, hasTag: boolean): boolean {
   if ((isMainBranch = hasTag)) {
      return true;
   }

   return false;
}
