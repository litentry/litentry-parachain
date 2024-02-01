
while getopts ":c:a:e:" opt; do
  case $opt in
    c) credentialId="$OPTARG"
    ;;
    a) address="$OPTARG"
    ;;
    e) expectValue="$OPTARG"
    ;;
    \?) echo "Invalid option -$OPTARG" >&2
    ;;
  esac
done

echo "credentialId: $credentialId"
echo "address: $address"
echo "expectValue: $expectValue"