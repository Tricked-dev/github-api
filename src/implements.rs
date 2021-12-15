use crate::{client::Client, end_points::*, Error};
use reqwest::Body;
use serde::{Deserialize, Serialize};
impl Client {
    pub async fn get_<T, V>(&self, query: Option<&T>, body: Option<V>) -> Result<GetResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::Get(), query, body).await
    }
    pub async fn get_app<T, V>(
        &self,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetAppResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetApp(), query, body).await
    }
    pub async fn get_app_hook_config<T, V>(
        &self,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetAppHookConfigResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetAppHookConfig(), query, body).await
    }
    pub async fn patch_app_hook_config<T, V>(
        &self,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PatchAppHookConfigResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::PatchAppHookConfig(), query, body).await
    }
    pub async fn get_app_hook_deliveries_delivery_id<T, V>(
        &self,
        delivery_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetAppHookDeliveriesdeliveryIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetAppHookDeliveriesdeliveryId(delivery_id),
            query,
            body,
        )
        .await
    }
    pub async fn get_app_installations_installation_id<T, V>(
        &self,
        installation_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetAppInstallationsinstallationIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetAppInstallationsinstallationId(installation_id),
            query,
            body,
        )
        .await
    }
    pub async fn get_applications_grants_grant_id<T, V>(
        &self,
        grant_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetApplicationsGrantsgrantIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetApplicationsGrantsgrantId(grant_id),
            query,
            body,
        )
        .await
    }
    pub async fn post_applications_client_id_token<T, V>(
        &self,
        client_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PostApplicationsclientIdTokenResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PostApplicationsclientIdToken(client_id),
            query,
            body,
        )
        .await
    }
    pub async fn patch_applications_client_id_token<T, V>(
        &self,
        client_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PatchApplicationsclientIdTokenResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchApplicationsclientIdToken(client_id),
            query,
            body,
        )
        .await
    }
    pub async fn post_applications_client_id_token_scoped<T, V>(
        &self,
        client_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PostApplicationsclientIdTokenScopedResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PostApplicationsclientIdTokenScoped(client_id),
            query,
            body,
        )
        .await
    }
    pub async fn get_apps_app_slug<T, V>(
        &self,
        app_slug: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetAppsappSlugResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetAppsappSlug(app_slug), query, body)
            .await
    }
    pub async fn put_authorizations_clients_client_id<T, V>(
        &self,
        client_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PutAuthorizationsClientsclientIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PutAuthorizationsClientsclientId(client_id),
            query,
            body,
        )
        .await
    }
    pub async fn put_authorizations_clients_client_id_fingerprint<T, V>(
        &self,
        client_id: String,
        fingerprint: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PutAuthorizationsClientsclientIdfingerprintResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PutAuthorizationsClientsclientIdfingerprint(client_id, fingerprint),
            query,
            body,
        )
        .await
    }
    pub async fn get_authorizations_authorization_id<T, V>(
        &self,
        authorization_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetAuthorizationsauthorizationIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetAuthorizationsauthorizationId(authorization_id),
            query,
            body,
        )
        .await
    }
    pub async fn patch_authorizations_authorization_id<T, V>(
        &self,
        authorization_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PatchAuthorizationsauthorizationIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchAuthorizationsauthorizationId(authorization_id),
            query,
            body,
        )
        .await
    }
    pub async fn get_codes_of_conduct_key<T, V>(
        &self,
        key: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetCodesOfConductkeyResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetCodesOfConductkey(key), query, body)
            .await
    }
    pub async fn get_enterprises_enterprise_actions_permissions<T, V>(
        &self,
        enterprise: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetEnterprisesenterpriseActionsPermissionsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetEnterprisesenterpriseActionsPermissions(enterprise),
            query,
            body,
        )
        .await
    }
    pub async fn get_enterprises_enterprise_actions_permissions_organizations<T, V>(
        &self,
        enterprise: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetEnterprisesenterpriseActionsPermissionsOrganizationsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetEnterprisesenterpriseActionsPermissionsOrganizations(enterprise),
            query,
            body,
        )
        .await
    }
    pub async fn get_enterprises_enterprise_actions_permissions_selected_actions<T, V>(
        &self,
        enterprise: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetEnterprisesenterpriseActionsPermissionsSelectedActionsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetEnterprisesenterpriseActionsPermissionsSelectedActions(enterprise),
            query,
            body,
        )
        .await
    }
    pub async fn get_enterprises_enterprise_actions_runner_groups<T, V>(
        &self,
        enterprise: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetEnterprisesenterpriseActionsRunnerGroupsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetEnterprisesenterpriseActionsRunnerGroups(enterprise),
            query,
            body,
        )
        .await
    }
    pub async fn get_enterprises_enterprise_actions_runner_groups_runner_group_id<T, V>(
        &self,
        enterprise: String,
        runner_group_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetEnterprisesenterpriseActionsRunnerGroupsrunnerGroupIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetEnterprisesenterpriseActionsRunnerGroupsrunnerGroupId(
                enterprise,
                runner_group_id,
            ),
            query,
            body,
        )
        .await
    }
    pub async fn patch_enterprises_enterprise_actions_runner_groups_runner_group_id<T, V>(
        &self,
        enterprise: String,
        runner_group_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PatchEnterprisesenterpriseActionsRunnerGroupsrunnerGroupIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchEnterprisesenterpriseActionsRunnerGroupsrunnerGroupId(
                enterprise,
                runner_group_id,
            ),
            query,
            body,
        )
        .await
    }
    pub async fn get_enterprises_enterprise_actions_runner_groups_runner_group_id_organizations<
        T,
        V,
    >(
        &self,
        enterprise: String,
        runner_group_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetEnterprisesenterpriseActionsRunnerGroupsrunnerGroupIdOrganizationsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetEnterprisesenterpriseActionsRunnerGroupsrunnerGroupIdOrganizations(
                enterprise,
                runner_group_id,
            ),
            query,
            body,
        )
        .await
    }
    pub async fn get_enterprises_enterprise_actions_runner_groups_runner_group_id_runners<T, V>(
        &self,
        enterprise: String,
        runner_group_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetEnterprisesenterpriseActionsRunnerGroupsrunnerGroupIdRunnersResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetEnterprisesenterpriseActionsRunnerGroupsrunnerGroupIdRunners(
                enterprise,
                runner_group_id,
            ),
            query,
            body,
        )
        .await
    }
    pub async fn get_enterprises_enterprise_actions_runners<T, V>(
        &self,
        enterprise: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetEnterprisesenterpriseActionsRunnersResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetEnterprisesenterpriseActionsRunners(enterprise),
            query,
            body,
        )
        .await
    }
    pub async fn get_enterprises_enterprise_actions_runners_runner_id<T, V>(
        &self,
        enterprise: String,
        runner_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetEnterprisesenterpriseActionsRunnersrunnerIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetEnterprisesenterpriseActionsRunnersrunnerId(enterprise, runner_id),
            query,
            body,
        )
        .await
    }
    pub async fn get_enterprises_enterprise_actions_runners_runner_id_labels<T, V>(
        &self,
        enterprise: String,
        runner_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetEnterprisesenterpriseActionsRunnersrunnerIdLabelsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetEnterprisesenterpriseActionsRunnersrunnerIdLabels(enterprise, runner_id),
            query,
            body,
        )
        .await
    }
    pub async fn post_enterprises_enterprise_actions_runners_runner_id_labels<T, V>(
        &self,
        enterprise: String,
        runner_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PostEnterprisesenterpriseActionsRunnersrunnerIdLabelsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PostEnterprisesenterpriseActionsRunnersrunnerIdLabels(enterprise, runner_id),
            query,
            body,
        )
        .await
    }
    pub async fn put_enterprises_enterprise_actions_runners_runner_id_labels<T, V>(
        &self,
        enterprise: String,
        runner_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PutEnterprisesenterpriseActionsRunnersrunnerIdLabelsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PutEnterprisesenterpriseActionsRunnersrunnerIdLabels(enterprise, runner_id),
            query,
            body,
        )
        .await
    }
    pub async fn delete_enterprises_enterprise_actions_runners_runner_id_labels<T, V>(
        &self,
        enterprise: String,
        runner_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<DeleteEnterprisesenterpriseActionsRunnersrunnerIdLabelsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::DeleteEnterprisesenterpriseActionsRunnersrunnerIdLabels(
                enterprise, runner_id,
            ),
            query,
            body,
        )
        .await
    }
    pub async fn delete_enterprises_enterprise_actions_runners_runner_id_labels_name<T, V>(
        &self,
        enterprise: String,
        runner_id: String,
        name: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<DeleteEnterprisesenterpriseActionsRunnersrunnerIdLabelsnameResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::DeleteEnterprisesenterpriseActionsRunnersrunnerIdLabelsname(
                enterprise, runner_id, name,
            ),
            query,
            body,
        )
        .await
    }
    pub async fn get_enterprises_enterprise_settings_billing_actions<T, V>(
        &self,
        enterprise: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetEnterprisesenterpriseSettingsBillingActionsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetEnterprisesenterpriseSettingsBillingActions(enterprise),
            query,
            body,
        )
        .await
    }
    pub async fn get_enterprises_enterprise_settings_billing_advanced_security<T, V>(
        &self,
        enterprise: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetEnterprisesenterpriseSettingsBillingAdvancedSecurityResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetEnterprisesenterpriseSettingsBillingAdvancedSecurity(enterprise),
            query,
            body,
        )
        .await
    }
    pub async fn get_enterprises_enterprise_settings_billing_packages<T, V>(
        &self,
        enterprise: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetEnterprisesenterpriseSettingsBillingPackagesResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetEnterprisesenterpriseSettingsBillingPackages(enterprise),
            query,
            body,
        )
        .await
    }
    pub async fn get_enterprises_enterprise_settings_billing_shared_storage<T, V>(
        &self,
        enterprise: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetEnterprisesenterpriseSettingsBillingSharedStorageResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetEnterprisesenterpriseSettingsBillingSharedStorage(enterprise),
            query,
            body,
        )
        .await
    }
    pub async fn get_feeds<T, V>(
        &self,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetFeedsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetFeeds(), query, body).await
    }
    pub async fn get_gists_gist_id<T, V>(
        &self,
        gist_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetGistsgistIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetGistsgistId(gist_id), query, body)
            .await
    }
    pub async fn patch_gists_gist_id<T, V>(
        &self,
        gist_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PatchGistsgistIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::PatchGistsgistId(gist_id), query, body)
            .await
    }
    pub async fn get_gists_gist_id_comments_comment_id<T, V>(
        &self,
        gist_id: String,
        comment_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetGistsgistIdCommentscommentIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetGistsgistIdCommentscommentId(gist_id, comment_id),
            query,
            body,
        )
        .await
    }
    pub async fn patch_gists_gist_id_comments_comment_id<T, V>(
        &self,
        gist_id: String,
        comment_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PatchGistsgistIdCommentscommentIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchGistsgistIdCommentscommentId(gist_id, comment_id),
            query,
            body,
        )
        .await
    }
    pub async fn get_gists_gist_id_sha<T, V>(
        &self,
        gist_id: String,
        sha: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetGistsgistIdshaResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetGistsgistIdsha(gist_id, sha), query, body)
            .await
    }
    pub async fn get_gitignore_templates_name<T, V>(
        &self,
        name: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetGitignoreTemplatesnameResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetGitignoreTemplatesname(name), query, body)
            .await
    }
    pub async fn get_installation_repositories<T, V>(
        &self,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetInstallationRepositoriesResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetInstallationRepositories(), query, body)
            .await
    }
    pub async fn get_licenses_license<T, V>(
        &self,
        license: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetLicenseslicenseResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetLicenseslicense(license), query, body)
            .await
    }
    pub async fn get_marketplace_listing_accounts_account_id<T, V>(
        &self,
        account_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetMarketplaceListingAccountsaccountIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetMarketplaceListingAccountsaccountId(account_id),
            query,
            body,
        )
        .await
    }
    pub async fn get_marketplace_listing_stubbed_accounts_account_id<T, V>(
        &self,
        account_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetMarketplaceListingStubbedAccountsaccountIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetMarketplaceListingStubbedAccountsaccountId(account_id),
            query,
            body,
        )
        .await
    }
    pub async fn get_meta<T, V>(
        &self,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetMetaResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetMeta(), query, body).await
    }
    pub async fn get_notifications_threads_thread_id<T, V>(
        &self,
        thread_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetNotificationsThreadsthreadIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetNotificationsThreadsthreadId(thread_id),
            query,
            body,
        )
        .await
    }
    pub async fn get_notifications_threads_thread_id_subscription<T, V>(
        &self,
        thread_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetNotificationsThreadsthreadIdSubscriptionResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetNotificationsThreadsthreadIdSubscription(thread_id),
            query,
            body,
        )
        .await
    }
    pub async fn put_notifications_threads_thread_id_subscription<T, V>(
        &self,
        thread_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PutNotificationsThreadsthreadIdSubscriptionResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PutNotificationsThreadsthreadIdSubscription(thread_id),
            query,
            body,
        )
        .await
    }
    pub async fn get_organizations_organization_id_custom_roles<T, V>(
        &self,
        organization_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetOrganizationsorganizationIdCustomRolesResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetOrganizationsorganizationIdCustomRoles(organization_id),
            query,
            body,
        )
        .await
    }
    pub async fn get_orgs_org<T, V>(
        &self,
        org: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetOrgsorgResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetOrgsorg(org), query, body).await
    }
    pub async fn patch_orgs_org<T, V>(
        &self,
        org: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PatchOrgsorgResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::PatchOrgsorg(org), query, body).await
    }
    pub async fn get_orgs_org_actions_permissions<T, V>(
        &self,
        org: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetOrgsorgActionsPermissionsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetOrgsorgActionsPermissions(org), query, body)
            .await
    }
    pub async fn get_orgs_org_actions_permissions_repositories<T, V>(
        &self,
        org: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetOrgsorgActionsPermissionsRepositoriesResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetOrgsorgActionsPermissionsRepositories(org),
            query,
            body,
        )
        .await
    }
    pub async fn get_orgs_org_actions_permissions_selected_actions<T, V>(
        &self,
        org: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetOrgsorgActionsPermissionsSelectedActionsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetOrgsorgActionsPermissionsSelectedActions(org),
            query,
            body,
        )
        .await
    }
    pub async fn get_orgs_org_actions_runner_groups<T, V>(
        &self,
        org: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetOrgsorgActionsRunnerGroupsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetOrgsorgActionsRunnerGroups(org), query, body)
            .await
    }
    pub async fn get_orgs_org_actions_runner_groups_runner_group_id<T, V>(
        &self,
        org: String,
        runner_group_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetOrgsorgActionsRunnerGroupsrunnerGroupIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetOrgsorgActionsRunnerGroupsrunnerGroupId(org, runner_group_id),
            query,
            body,
        )
        .await
    }
    pub async fn patch_orgs_org_actions_runner_groups_runner_group_id<T, V>(
        &self,
        org: String,
        runner_group_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PatchOrgsorgActionsRunnerGroupsrunnerGroupIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchOrgsorgActionsRunnerGroupsrunnerGroupId(org, runner_group_id),
            query,
            body,
        )
        .await
    }
    pub async fn get_orgs_org_actions_runner_groups_runner_group_id_repositories<T, V>(
        &self,
        org: String,
        runner_group_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetOrgsorgActionsRunnerGroupsrunnerGroupIdRepositoriesResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetOrgsorgActionsRunnerGroupsrunnerGroupIdRepositories(org, runner_group_id),
            query,
            body,
        )
        .await
    }
    pub async fn get_orgs_org_actions_runner_groups_runner_group_id_runners<T, V>(
        &self,
        org: String,
        runner_group_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetOrgsorgActionsRunnerGroupsrunnerGroupIdRunnersResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetOrgsorgActionsRunnerGroupsrunnerGroupIdRunners(org, runner_group_id),
            query,
            body,
        )
        .await
    }
    pub async fn get_orgs_org_actions_runners<T, V>(
        &self,
        org: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetOrgsorgActionsRunnersResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetOrgsorgActionsRunners(org), query, body)
            .await
    }
    pub async fn get_orgs_org_actions_runners_runner_id<T, V>(
        &self,
        org: String,
        runner_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetOrgsorgActionsRunnersrunnerIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetOrgsorgActionsRunnersrunnerId(org, runner_id),
            query,
            body,
        )
        .await
    }
    pub async fn get_orgs_org_actions_runners_runner_id_labels<T, V>(
        &self,
        org: String,
        runner_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetOrgsorgActionsRunnersrunnerIdLabelsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetOrgsorgActionsRunnersrunnerIdLabels(org, runner_id),
            query,
            body,
        )
        .await
    }
    pub async fn post_orgs_org_actions_runners_runner_id_labels<T, V>(
        &self,
        org: String,
        runner_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PostOrgsorgActionsRunnersrunnerIdLabelsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PostOrgsorgActionsRunnersrunnerIdLabels(org, runner_id),
            query,
            body,
        )
        .await
    }
    pub async fn put_orgs_org_actions_runners_runner_id_labels<T, V>(
        &self,
        org: String,
        runner_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PutOrgsorgActionsRunnersrunnerIdLabelsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PutOrgsorgActionsRunnersrunnerIdLabels(org, runner_id),
            query,
            body,
        )
        .await
    }
    pub async fn delete_orgs_org_actions_runners_runner_id_labels<T, V>(
        &self,
        org: String,
        runner_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<DeleteOrgsorgActionsRunnersrunnerIdLabelsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::DeleteOrgsorgActionsRunnersrunnerIdLabels(org, runner_id),
            query,
            body,
        )
        .await
    }
    pub async fn delete_orgs_org_actions_runners_runner_id_labels_name<T, V>(
        &self,
        org: String,
        runner_id: String,
        name: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<DeleteOrgsorgActionsRunnersrunnerIdLabelsnameResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::DeleteOrgsorgActionsRunnersrunnerIdLabelsname(org, runner_id, name),
            query,
            body,
        )
        .await
    }
    pub async fn get_orgs_org_actions_secrets<T, V>(
        &self,
        org: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetOrgsorgActionsSecretsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetOrgsorgActionsSecrets(org), query, body)
            .await
    }
    pub async fn get_orgs_org_actions_secrets_public_key<T, V>(
        &self,
        org: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetOrgsorgActionsSecretsPublicKeyResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetOrgsorgActionsSecretsPublicKey(org),
            query,
            body,
        )
        .await
    }
    pub async fn get_orgs_org_actions_secrets_secret_name<T, V>(
        &self,
        org: String,
        secret_name: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetOrgsorgActionsSecretssecretNameResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetOrgsorgActionsSecretssecretName(org, secret_name),
            query,
            body,
        )
        .await
    }
    pub async fn get_orgs_org_actions_secrets_secret_name_repositories<T, V>(
        &self,
        org: String,
        secret_name: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetOrgsorgActionsSecretssecretNameRepositoriesResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetOrgsorgActionsSecretssecretNameRepositories(org, secret_name),
            query,
            body,
        )
        .await
    }
    pub async fn get_orgs_org_external_group_group_id<T, V>(
        &self,
        org: String,
        group_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetOrgsorgExternalGroupgroupIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetOrgsorgExternalGroupgroupId(org, group_id),
            query,
            body,
        )
        .await
    }
    pub async fn get_orgs_org_external_groups<T, V>(
        &self,
        org: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetOrgsorgExternalGroupsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetOrgsorgExternalGroups(org), query, body)
            .await
    }
    pub async fn get_orgs_org_hooks_hook_id<T, V>(
        &self,
        org: String,
        hook_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetOrgsorgHookshookIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetOrgsorgHookshookId(org, hook_id), query, body)
            .await
    }
    pub async fn patch_orgs_org_hooks_hook_id<T, V>(
        &self,
        org: String,
        hook_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PatchOrgsorgHookshookIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchOrgsorgHookshookId(org, hook_id),
            query,
            body,
        )
        .await
    }
    pub async fn get_orgs_org_hooks_hook_id_config<T, V>(
        &self,
        org: String,
        hook_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetOrgsorgHookshookIdConfigResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetOrgsorgHookshookIdConfig(org, hook_id),
            query,
            body,
        )
        .await
    }
    pub async fn patch_orgs_org_hooks_hook_id_config<T, V>(
        &self,
        org: String,
        hook_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PatchOrgsorgHookshookIdConfigResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchOrgsorgHookshookIdConfig(org, hook_id),
            query,
            body,
        )
        .await
    }
    pub async fn get_orgs_org_hooks_hook_id_deliveries_delivery_id<T, V>(
        &self,
        org: String,
        hook_id: String,
        delivery_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetOrgsorgHookshookIdDeliveriesdeliveryIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetOrgsorgHookshookIdDeliveriesdeliveryId(org, hook_id, delivery_id),
            query,
            body,
        )
        .await
    }
    pub async fn get_orgs_org_installation<T, V>(
        &self,
        org: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetOrgsorgInstallationResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetOrgsorgInstallation(org), query, body)
            .await
    }
    pub async fn get_orgs_org_installations<T, V>(
        &self,
        org: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetOrgsorgInstallationsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetOrgsorgInstallations(org), query, body)
            .await
    }
    pub async fn put_orgs_org_interaction_limits<T, V>(
        &self,
        org: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PutOrgsorgInteractionLimitsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::PutOrgsorgInteractionLimits(org), query, body)
            .await
    }
    pub async fn get_orgs_org_memberships_username<T, V>(
        &self,
        org: String,
        username: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetOrgsorgMembershipsusernameResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetOrgsorgMembershipsusername(org, username),
            query,
            body,
        )
        .await
    }
    pub async fn put_orgs_org_memberships_username<T, V>(
        &self,
        org: String,
        username: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PutOrgsorgMembershipsusernameResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PutOrgsorgMembershipsusername(org, username),
            query,
            body,
        )
        .await
    }
    pub async fn get_orgs_org_migrations_migration_id<T, V>(
        &self,
        org: String,
        migration_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetOrgsorgMigrationsmigrationIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetOrgsorgMigrationsmigrationId(org, migration_id),
            query,
            body,
        )
        .await
    }
    pub async fn get_orgs_org_packages_package_type_package_name<T, V>(
        &self,
        org: String,
        package_type: String,
        package_name: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetOrgsorgPackagespackageTypepackageNameResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetOrgsorgPackagespackageTypepackageName(org, package_type, package_name),
            query,
            body,
        )
        .await
    }
    pub async fn get_orgs_org_packages_package_type_package_name_versions_package_version_id<T, V>(
        &self,
        org: String,
        package_type: String,
        package_name: String,
        package_version_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetOrgsorgPackagespackageTypepackageNameVersionspackageVersionIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetOrgsorgPackagespackageTypepackageNameVersionspackageVersionId(
                org,
                package_type,
                package_name,
                package_version_id,
            ),
            query,
            body,
        )
        .await
    }
    pub async fn get_orgs_org_settings_billing_actions<T, V>(
        &self,
        org: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetOrgsorgSettingsBillingActionsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetOrgsorgSettingsBillingActions(org),
            query,
            body,
        )
        .await
    }
    pub async fn get_orgs_org_settings_billing_advanced_security<T, V>(
        &self,
        org: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetOrgsorgSettingsBillingAdvancedSecurityResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetOrgsorgSettingsBillingAdvancedSecurity(org),
            query,
            body,
        )
        .await
    }
    pub async fn get_orgs_org_settings_billing_packages<T, V>(
        &self,
        org: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetOrgsorgSettingsBillingPackagesResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetOrgsorgSettingsBillingPackages(org),
            query,
            body,
        )
        .await
    }
    pub async fn get_orgs_org_settings_billing_shared_storage<T, V>(
        &self,
        org: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetOrgsorgSettingsBillingSharedStorageResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetOrgsorgSettingsBillingSharedStorage(org),
            query,
            body,
        )
        .await
    }
    pub async fn get_orgs_org_team_sync_groups<T, V>(
        &self,
        org: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetOrgsorgTeamSyncGroupsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetOrgsorgTeamSyncGroups(org), query, body)
            .await
    }
    pub async fn get_orgs_org_teams_team_slug<T, V>(
        &self,
        org: String,
        team_slug: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetOrgsorgTeamsteamSlugResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetOrgsorgTeamsteamSlug(org, team_slug),
            query,
            body,
        )
        .await
    }
    pub async fn get_orgs_org_teams_team_slug_discussions_discussion_number<T, V>(
        &self,
        org: String,
        team_slug: String,
        discussion_number: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetOrgsorgTeamsteamSlugDiscussionsdiscussionNumberResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetOrgsorgTeamsteamSlugDiscussionsdiscussionNumber(
                org,
                team_slug,
                discussion_number,
            ),
            query,
            body,
        )
        .await
    }
    pub async fn patch_orgs_org_teams_team_slug_discussions_discussion_number<T, V>(
        &self,
        org: String,
        team_slug: String,
        discussion_number: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PatchOrgsorgTeamsteamSlugDiscussionsdiscussionNumberResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchOrgsorgTeamsteamSlugDiscussionsdiscussionNumber(
                org,
                team_slug,
                discussion_number,
            ),
            query,
            body,
        )
        .await
    }
    pub async fn get_orgs_org_teams_team_slug_discussions_discussion_number_comments_comment_number<
        T,
        V,
    >(
        &self,
        org: String,
        team_slug: String,
        discussion_number: String,
        comment_number: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<
        GetOrgsorgTeamsteamSlugDiscussionsdiscussionNumberCommentscommentNumberResponse,
        Error,
    >
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetOrgsorgTeamsteamSlugDiscussionsdiscussionNumberCommentscommentNumber(
                org,
                team_slug,
                discussion_number,
                comment_number,
            ),
            query,
            body,
        )
        .await
    }
    pub async fn patch_orgs_org_teams_team_slug_discussions_discussion_number_comments_comment_number<
        T,
        V,
    >(
        &self,
        org: String,
        team_slug: String,
        discussion_number: String,
        comment_number: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<
        PatchOrgsorgTeamsteamSlugDiscussionsdiscussionNumberCommentscommentNumberResponse,
        Error,
    >
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchOrgsorgTeamsteamSlugDiscussionsdiscussionNumberCommentscommentNumber(
                org,
                team_slug,
                discussion_number,
                comment_number,
            ),
            query,
            body,
        )
        .await
    }
    pub async fn post_orgs_org_teams_team_slug_discussions_discussion_number_comments_comment_number_reactions<
        T,
        V,
    >(
        &self,
        org: String,
        team_slug: String,
        discussion_number: String,
        comment_number: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<
        PostOrgsorgTeamsteamSlugDiscussionsdiscussionNumberCommentscommentNumberReactionsResponse,
        Error,
    >
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::PostOrgsorgTeamsteamSlugDiscussionsdiscussionNumberCommentscommentNumberReactions(org, team_slug, discussion_number, comment_number), query, body).await
    }
    pub async fn post_orgs_org_teams_team_slug_discussions_discussion_number_reactions<T, V>(
        &self,
        org: String,
        team_slug: String,
        discussion_number: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PostOrgsorgTeamsteamSlugDiscussionsdiscussionNumberReactionsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PostOrgsorgTeamsteamSlugDiscussionsdiscussionNumberReactions(
                org,
                team_slug,
                discussion_number,
            ),
            query,
            body,
        )
        .await
    }
    pub async fn patch_orgs_org_teams_team_slug_external_groups<T, V>(
        &self,
        org: String,
        team_slug: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PatchOrgsorgTeamsteamSlugExternalGroupsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchOrgsorgTeamsteamSlugExternalGroups(org, team_slug),
            query,
            body,
        )
        .await
    }
    pub async fn get_orgs_org_teams_team_slug_memberships_username<T, V>(
        &self,
        org: String,
        team_slug: String,
        username: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetOrgsorgTeamsteamSlugMembershipsusernameResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetOrgsorgTeamsteamSlugMembershipsusername(org, team_slug, username),
            query,
            body,
        )
        .await
    }
    pub async fn put_orgs_org_teams_team_slug_memberships_username<T, V>(
        &self,
        org: String,
        team_slug: String,
        username: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PutOrgsorgTeamsteamSlugMembershipsusernameResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PutOrgsorgTeamsteamSlugMembershipsusername(org, team_slug, username),
            query,
            body,
        )
        .await
    }
    pub async fn get_orgs_org_teams_team_slug_projects_project_id<T, V>(
        &self,
        org: String,
        team_slug: String,
        project_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetOrgsorgTeamsteamSlugProjectsprojectIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetOrgsorgTeamsteamSlugProjectsprojectId(org, team_slug, project_id),
            query,
            body,
        )
        .await
    }
    pub async fn get_orgs_org_teams_team_slug_repos_owner_repo<T, V>(
        &self,
        org: String,
        team_slug: String,
        owner: String,
        repo: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetOrgsorgTeamsteamSlugReposownerrepoResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetOrgsorgTeamsteamSlugReposownerrepo(org, team_slug, owner, repo),
            query,
            body,
        )
        .await
    }
    pub async fn get_orgs_org_teams_team_slug_team_sync_group_mappings<T, V>(
        &self,
        org: String,
        team_slug: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetOrgsorgTeamsteamSlugTeamSyncGroupMappingsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetOrgsorgTeamsteamSlugTeamSyncGroupMappings(org, team_slug),
            query,
            body,
        )
        .await
    }
    pub async fn patch_orgs_org_teams_team_slug_team_sync_group_mappings<T, V>(
        &self,
        org: String,
        team_slug: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PatchOrgsorgTeamsteamSlugTeamSyncGroupMappingsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchOrgsorgTeamsteamSlugTeamSyncGroupMappings(org, team_slug),
            query,
            body,
        )
        .await
    }
    pub async fn get_projects_columns_cards_card_id<T, V>(
        &self,
        card_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetProjectsColumnsCardscardIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetProjectsColumnsCardscardId(card_id),
            query,
            body,
        )
        .await
    }
    pub async fn patch_projects_columns_cards_card_id<T, V>(
        &self,
        card_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PatchProjectsColumnsCardscardIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchProjectsColumnsCardscardId(card_id),
            query,
            body,
        )
        .await
    }
    pub async fn get_projects_columns_column_id<T, V>(
        &self,
        column_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetProjectsColumnscolumnIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetProjectsColumnscolumnId(column_id),
            query,
            body,
        )
        .await
    }
    pub async fn patch_projects_columns_column_id<T, V>(
        &self,
        column_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PatchProjectsColumnscolumnIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchProjectsColumnscolumnId(column_id),
            query,
            body,
        )
        .await
    }
    pub async fn get_projects_project_id<T, V>(
        &self,
        project_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetProjectsprojectIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetProjectsprojectId(project_id), query, body)
            .await
    }
    pub async fn patch_projects_project_id<T, V>(
        &self,
        project_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PatchProjectsprojectIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::PatchProjectsprojectId(project_id), query, body)
            .await
    }
    pub async fn get_projects_project_id_collaborators_username_permission<T, V>(
        &self,
        project_id: String,
        username: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetProjectsprojectIdCollaboratorsusernamePermissionResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetProjectsprojectIdCollaboratorsusernamePermission(project_id, username),
            query,
            body,
        )
        .await
    }
    pub async fn get_rate_limit<T, V>(
        &self,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetRateLimitResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetRateLimit(), query, body).await
    }
    pub async fn get_repos_owner_repo<T, V>(
        &self,
        owner: String,
        repo: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetReposownerrepo(owner, repo), query, body)
            .await
    }
    pub async fn patch_repos_owner_repo<T, V>(
        &self,
        owner: String,
        repo: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PatchReposownerrepoResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::PatchReposownerrepo(owner, repo), query, body)
            .await
    }
    pub async fn get_repos_owner_repo_actions_artifacts<T, V>(
        &self,
        owner: String,
        repo: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoActionsArtifactsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoActionsArtifacts(owner, repo),
            query,
            body,
        )
        .await
    }
    pub async fn get_repos_owner_repo_actions_artifacts_artifact_id<T, V>(
        &self,
        owner: String,
        repo: String,
        artifact_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoActionsArtifactsartifactIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoActionsArtifactsartifactId(owner, repo, artifact_id),
            query,
            body,
        )
        .await
    }
    pub async fn get_repos_owner_repo_actions_jobs_job_id<T, V>(
        &self,
        owner: String,
        repo: String,
        job_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoActionsJobsjobIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoActionsJobsjobId(owner, repo, job_id),
            query,
            body,
        )
        .await
    }
    pub async fn get_repos_owner_repo_actions_permissions<T, V>(
        &self,
        owner: String,
        repo: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoActionsPermissionsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoActionsPermissions(owner, repo),
            query,
            body,
        )
        .await
    }
    pub async fn get_repos_owner_repo_actions_permissions_selected_actions<T, V>(
        &self,
        owner: String,
        repo: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoActionsPermissionsSelectedActionsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoActionsPermissionsSelectedActions(owner, repo),
            query,
            body,
        )
        .await
    }
    pub async fn get_repos_owner_repo_actions_runners<T, V>(
        &self,
        owner: String,
        repo: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoActionsRunnersResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoActionsRunners(owner, repo),
            query,
            body,
        )
        .await
    }
    pub async fn get_repos_owner_repo_actions_runners_runner_id<T, V>(
        &self,
        owner: String,
        repo: String,
        runner_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoActionsRunnersrunnerIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoActionsRunnersrunnerId(owner, repo, runner_id),
            query,
            body,
        )
        .await
    }
    pub async fn get_repos_owner_repo_actions_runners_runner_id_labels<T, V>(
        &self,
        owner: String,
        repo: String,
        runner_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoActionsRunnersrunnerIdLabelsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoActionsRunnersrunnerIdLabels(owner, repo, runner_id),
            query,
            body,
        )
        .await
    }
    pub async fn post_repos_owner_repo_actions_runners_runner_id_labels<T, V>(
        &self,
        owner: String,
        repo: String,
        runner_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PostReposownerrepoActionsRunnersrunnerIdLabelsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PostReposownerrepoActionsRunnersrunnerIdLabels(owner, repo, runner_id),
            query,
            body,
        )
        .await
    }
    pub async fn put_repos_owner_repo_actions_runners_runner_id_labels<T, V>(
        &self,
        owner: String,
        repo: String,
        runner_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PutReposownerrepoActionsRunnersrunnerIdLabelsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PutReposownerrepoActionsRunnersrunnerIdLabels(owner, repo, runner_id),
            query,
            body,
        )
        .await
    }
    pub async fn delete_repos_owner_repo_actions_runners_runner_id_labels<T, V>(
        &self,
        owner: String,
        repo: String,
        runner_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<DeleteReposownerrepoActionsRunnersrunnerIdLabelsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::DeleteReposownerrepoActionsRunnersrunnerIdLabels(owner, repo, runner_id),
            query,
            body,
        )
        .await
    }
    pub async fn delete_repos_owner_repo_actions_runners_runner_id_labels_name<T, V>(
        &self,
        owner: String,
        repo: String,
        runner_id: String,
        name: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<DeleteReposownerrepoActionsRunnersrunnerIdLabelsnameResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::DeleteReposownerrepoActionsRunnersrunnerIdLabelsname(
                owner, repo, runner_id, name,
            ),
            query,
            body,
        )
        .await
    }
    pub async fn get_repos_owner_repo_actions_runs<T, V>(
        &self,
        owner: String,
        repo: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoActionsRunsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoActionsRuns(owner, repo),
            query,
            body,
        )
        .await
    }
    pub async fn get_repos_owner_repo_actions_runs_run_id<T, V>(
        &self,
        owner: String,
        repo: String,
        run_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoActionsRunsrunIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoActionsRunsrunId(owner, repo, run_id),
            query,
            body,
        )
        .await
    }
    pub async fn get_repos_owner_repo_actions_runs_run_id_artifacts<T, V>(
        &self,
        owner: String,
        repo: String,
        run_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoActionsRunsrunIdArtifactsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoActionsRunsrunIdArtifacts(owner, repo, run_id),
            query,
            body,
        )
        .await
    }
    pub async fn get_repos_owner_repo_actions_runs_run_id_attempts_attempt_number<T, V>(
        &self,
        owner: String,
        repo: String,
        run_id: String,
        attempt_number: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoActionsRunsrunIdAttemptsattemptNumberResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoActionsRunsrunIdAttemptsattemptNumber(
                owner,
                repo,
                run_id,
                attempt_number,
            ),
            query,
            body,
        )
        .await
    }
    pub async fn get_repos_owner_repo_actions_runs_run_id_attempts_attempt_number_jobs<T, V>(
        &self,
        owner: String,
        repo: String,
        run_id: String,
        attempt_number: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoActionsRunsrunIdAttemptsattemptNumberJobsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoActionsRunsrunIdAttemptsattemptNumberJobs(
                owner,
                repo,
                run_id,
                attempt_number,
            ),
            query,
            body,
        )
        .await
    }
    pub async fn get_repos_owner_repo_actions_runs_run_id_jobs<T, V>(
        &self,
        owner: String,
        repo: String,
        run_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoActionsRunsrunIdJobsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoActionsRunsrunIdJobs(owner, repo, run_id),
            query,
            body,
        )
        .await
    }
    pub async fn get_repos_owner_repo_actions_runs_run_id_timing<T, V>(
        &self,
        owner: String,
        repo: String,
        run_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoActionsRunsrunIdTimingResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoActionsRunsrunIdTiming(owner, repo, run_id),
            query,
            body,
        )
        .await
    }
    pub async fn get_repos_owner_repo_actions_secrets<T, V>(
        &self,
        owner: String,
        repo: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoActionsSecretsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoActionsSecrets(owner, repo),
            query,
            body,
        )
        .await
    }
    pub async fn get_repos_owner_repo_actions_secrets_public_key<T, V>(
        &self,
        owner: String,
        repo: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoActionsSecretsPublicKeyResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoActionsSecretsPublicKey(owner, repo),
            query,
            body,
        )
        .await
    }
    pub async fn get_repos_owner_repo_actions_secrets_secret_name<T, V>(
        &self,
        owner: String,
        repo: String,
        secret_name: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoActionsSecretssecretNameResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoActionsSecretssecretName(owner, repo, secret_name),
            query,
            body,
        )
        .await
    }
    pub async fn get_repos_owner_repo_actions_workflows<T, V>(
        &self,
        owner: String,
        repo: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoActionsWorkflowsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoActionsWorkflows(owner, repo),
            query,
            body,
        )
        .await
    }
    pub async fn get_repos_owner_repo_actions_workflows_workflow_id<T, V>(
        &self,
        owner: String,
        repo: String,
        workflow_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoActionsWorkflowsworkflowIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoActionsWorkflowsworkflowId(owner, repo, workflow_id),
            query,
            body,
        )
        .await
    }
    pub async fn get_repos_owner_repo_actions_workflows_workflow_id_runs<T, V>(
        &self,
        owner: String,
        repo: String,
        workflow_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoActionsWorkflowsworkflowIdRunsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoActionsWorkflowsworkflowIdRuns(owner, repo, workflow_id),
            query,
            body,
        )
        .await
    }
    pub async fn get_repos_owner_repo_actions_workflows_workflow_id_timing<T, V>(
        &self,
        owner: String,
        repo: String,
        workflow_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoActionsWorkflowsworkflowIdTimingResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoActionsWorkflowsworkflowIdTiming(owner, repo, workflow_id),
            query,
            body,
        )
        .await
    }
    pub async fn get_repos_owner_repo_autolinks_autolink_id<T, V>(
        &self,
        owner: String,
        repo: String,
        autolink_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoAutolinksautolinkIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoAutolinksautolinkId(owner, repo, autolink_id),
            query,
            body,
        )
        .await
    }
    pub async fn get_repos_owner_repo_branches_branch<T, V>(
        &self,
        owner: String,
        repo: String,
        branch: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoBranchesbranchResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoBranchesbranch(owner, repo, branch),
            query,
            body,
        )
        .await
    }
    pub async fn get_repos_owner_repo_branches_branch_protection<T, V>(
        &self,
        owner: String,
        repo: String,
        branch: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoBranchesbranchProtectionResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoBranchesbranchProtection(owner, repo, branch),
            query,
            body,
        )
        .await
    }
    pub async fn put_repos_owner_repo_branches_branch_protection<T, V>(
        &self,
        owner: String,
        repo: String,
        branch: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PutReposownerrepoBranchesbranchProtectionResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PutReposownerrepoBranchesbranchProtection(owner, repo, branch),
            query,
            body,
        )
        .await
    }
    pub async fn get_repos_owner_repo_branches_branch_protection_enforce_admins<T, V>(
        &self,
        owner: String,
        repo: String,
        branch: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoBranchesbranchProtectionEnforceAdminsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoBranchesbranchProtectionEnforceAdmins(owner, repo, branch),
            query,
            body,
        )
        .await
    }
    pub async fn post_repos_owner_repo_branches_branch_protection_enforce_admins<T, V>(
        &self,
        owner: String,
        repo: String,
        branch: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PostReposownerrepoBranchesbranchProtectionEnforceAdminsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PostReposownerrepoBranchesbranchProtectionEnforceAdmins(owner, repo, branch),
            query,
            body,
        )
        .await
    }
    pub async fn get_repos_owner_repo_branches_branch_protection_required_pull_request_reviews<
        T,
        V,
    >(
        &self,
        owner: String,
        repo: String,
        branch: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoBranchesbranchProtectionRequiredPullRequestReviewsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoBranchesbranchProtectionRequiredPullRequestReviews(
                owner, repo, branch,
            ),
            query,
            body,
        )
        .await
    }
    pub async fn patch_repos_owner_repo_branches_branch_protection_required_pull_request_reviews<
        T,
        V,
    >(
        &self,
        owner: String,
        repo: String,
        branch: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PatchReposownerrepoBranchesbranchProtectionRequiredPullRequestReviewsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchReposownerrepoBranchesbranchProtectionRequiredPullRequestReviews(
                owner, repo, branch,
            ),
            query,
            body,
        )
        .await
    }
    pub async fn get_repos_owner_repo_branches_branch_protection_required_signatures<T, V>(
        &self,
        owner: String,
        repo: String,
        branch: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoBranchesbranchProtectionRequiredSignaturesResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoBranchesbranchProtectionRequiredSignatures(
                owner, repo, branch,
            ),
            query,
            body,
        )
        .await
    }
    pub async fn post_repos_owner_repo_branches_branch_protection_required_signatures<T, V>(
        &self,
        owner: String,
        repo: String,
        branch: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PostReposownerrepoBranchesbranchProtectionRequiredSignaturesResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PostReposownerrepoBranchesbranchProtectionRequiredSignatures(
                owner, repo, branch,
            ),
            query,
            body,
        )
        .await
    }
    pub async fn get_repos_owner_repo_branches_branch_protection_required_status_checks<T, V>(
        &self,
        owner: String,
        repo: String,
        branch: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoBranchesbranchProtectionRequiredStatusChecksResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoBranchesbranchProtectionRequiredStatusChecks(
                owner, repo, branch,
            ),
            query,
            body,
        )
        .await
    }
    pub async fn patch_repos_owner_repo_branches_branch_protection_required_status_checks<T, V>(
        &self,
        owner: String,
        repo: String,
        branch: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PatchReposownerrepoBranchesbranchProtectionRequiredStatusChecksResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchReposownerrepoBranchesbranchProtectionRequiredStatusChecks(
                owner, repo, branch,
            ),
            query,
            body,
        )
        .await
    }
    pub async fn get_repos_owner_repo_branches_branch_protection_restrictions<T, V>(
        &self,
        owner: String,
        repo: String,
        branch: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoBranchesbranchProtectionRestrictionsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoBranchesbranchProtectionRestrictions(owner, repo, branch),
            query,
            body,
        )
        .await
    }
    pub async fn get_repos_owner_repo_check_runs_check_run_id<T, V>(
        &self,
        owner: String,
        repo: String,
        check_run_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoCheckRunscheckRunIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoCheckRunscheckRunId(owner, repo, check_run_id),
            query,
            body,
        )
        .await
    }
    pub async fn patch_repos_owner_repo_check_runs_check_run_id<T, V>(
        &self,
        owner: String,
        repo: String,
        check_run_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PatchReposownerrepoCheckRunscheckRunIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchReposownerrepoCheckRunscheckRunId(owner, repo, check_run_id),
            query,
            body,
        )
        .await
    }
    pub async fn post_repos_owner_repo_check_suites<T, V>(
        &self,
        owner: String,
        repo: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PostReposownerrepoCheckSuitesResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PostReposownerrepoCheckSuites(owner, repo),
            query,
            body,
        )
        .await
    }
    pub async fn patch_repos_owner_repo_check_suites_preferences<T, V>(
        &self,
        owner: String,
        repo: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PatchReposownerrepoCheckSuitesPreferencesResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchReposownerrepoCheckSuitesPreferences(owner, repo),
            query,
            body,
        )
        .await
    }
    pub async fn get_repos_owner_repo_check_suites_check_suite_id<T, V>(
        &self,
        owner: String,
        repo: String,
        check_suite_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoCheckSuitescheckSuiteIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoCheckSuitescheckSuiteId(owner, repo, check_suite_id),
            query,
            body,
        )
        .await
    }
    pub async fn get_repos_owner_repo_check_suites_check_suite_id_check_runs<T, V>(
        &self,
        owner: String,
        repo: String,
        check_suite_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoCheckSuitescheckSuiteIdCheckRunsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoCheckSuitescheckSuiteIdCheckRuns(
                owner,
                repo,
                check_suite_id,
            ),
            query,
            body,
        )
        .await
    }
    pub async fn get_repos_owner_repo_code_scanning_alerts_alert_number<T, V>(
        &self,
        owner: String,
        repo: String,
        alert_number: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoCodeScanningAlertsalertNumberResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoCodeScanningAlertsalertNumber(owner, repo, alert_number),
            query,
            body,
        )
        .await
    }
    pub async fn patch_repos_owner_repo_code_scanning_alerts_alert_number<T, V>(
        &self,
        owner: String,
        repo: String,
        alert_number: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PatchReposownerrepoCodeScanningAlertsalertNumberResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchReposownerrepoCodeScanningAlertsalertNumber(owner, repo, alert_number),
            query,
            body,
        )
        .await
    }
    pub async fn get_repos_owner_repo_code_scanning_analyses_analysis_id<T, V>(
        &self,
        owner: String,
        repo: String,
        analysis_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoCodeScanningAnalysesanalysisIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoCodeScanningAnalysesanalysisId(owner, repo, analysis_id),
            query,
            body,
        )
        .await
    }
    pub async fn delete_repos_owner_repo_code_scanning_analyses_analysis_id<T, V>(
        &self,
        owner: String,
        repo: String,
        analysis_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<DeleteReposownerrepoCodeScanningAnalysesanalysisIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::DeleteReposownerrepoCodeScanningAnalysesanalysisId(owner, repo, analysis_id),
            query,
            body,
        )
        .await
    }
    pub async fn get_repos_owner_repo_code_scanning_sarifs_sarif_id<T, V>(
        &self,
        owner: String,
        repo: String,
        sarif_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoCodeScanningSarifssarifIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoCodeScanningSarifssarifId(owner, repo, sarif_id),
            query,
            body,
        )
        .await
    }
    pub async fn get_repos_owner_repo_codespaces<T, V>(
        &self,
        owner: String,
        repo: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoCodespacesResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoCodespaces(owner, repo),
            query,
            body,
        )
        .await
    }
    pub async fn get_repos_owner_repo_codespaces_machines<T, V>(
        &self,
        owner: String,
        repo: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoCodespacesMachinesResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoCodespacesMachines(owner, repo),
            query,
            body,
        )
        .await
    }
    pub async fn get_repos_owner_repo_collaborators_username_permission<T, V>(
        &self,
        owner: String,
        repo: String,
        username: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoCollaboratorsusernamePermissionResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoCollaboratorsusernamePermission(owner, repo, username),
            query,
            body,
        )
        .await
    }
    pub async fn get_repos_owner_repo_comments_comment_id<T, V>(
        &self,
        owner: String,
        repo: String,
        comment_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoCommentscommentIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoCommentscommentId(owner, repo, comment_id),
            query,
            body,
        )
        .await
    }
    pub async fn patch_repos_owner_repo_comments_comment_id<T, V>(
        &self,
        owner: String,
        repo: String,
        comment_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PatchReposownerrepoCommentscommentIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchReposownerrepoCommentscommentId(owner, repo, comment_id),
            query,
            body,
        )
        .await
    }
    pub async fn post_repos_owner_repo_comments_comment_id_reactions<T, V>(
        &self,
        owner: String,
        repo: String,
        comment_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PostReposownerrepoCommentscommentIdReactionsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PostReposownerrepoCommentscommentIdReactions(owner, repo, comment_id),
            query,
            body,
        )
        .await
    }
    pub async fn get_repos_owner_repo_commits_aref<T, V>(
        &self,
        owner: String,
        repo: String,
        aref: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoCommitsrefResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoCommitsref(owner, repo, aref),
            query,
            body,
        )
        .await
    }
    pub async fn get_repos_owner_repo_commits_aref_check_runs<T, V>(
        &self,
        owner: String,
        repo: String,
        aref: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoCommitsrefCheckRunsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoCommitsrefCheckRuns(owner, repo, aref),
            query,
            body,
        )
        .await
    }
    pub async fn get_repos_owner_repo_commits_aref_check_suites<T, V>(
        &self,
        owner: String,
        repo: String,
        aref: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoCommitsrefCheckSuitesResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoCommitsrefCheckSuites(owner, repo, aref),
            query,
            body,
        )
        .await
    }
    pub async fn get_repos_owner_repo_commits_aref_status<T, V>(
        &self,
        owner: String,
        repo: String,
        aref: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoCommitsrefStatusResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoCommitsrefStatus(owner, repo, aref),
            query,
            body,
        )
        .await
    }
    pub async fn get_repos_owner_repo_community_profile<T, V>(
        &self,
        owner: String,
        repo: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoCommunityProfileResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoCommunityProfile(owner, repo),
            query,
            body,
        )
        .await
    }
    pub async fn get_repos_owner_repo_compare_basehead<T, V>(
        &self,
        owner: String,
        repo: String,
        basehead: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoComparebaseheadResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoComparebasehead(owner, repo, basehead),
            query,
            body,
        )
        .await
    }
    pub async fn put_repos_owner_repo_contents_path<T, V>(
        &self,
        owner: String,
        repo: String,
        path: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PutReposownerrepoContentspathResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PutReposownerrepoContentspath(owner, repo, path),
            query,
            body,
        )
        .await
    }
    pub async fn delete_repos_owner_repo_contents_path<T, V>(
        &self,
        owner: String,
        repo: String,
        path: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<DeleteReposownerrepoContentspathResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::DeleteReposownerrepoContentspath(owner, repo, path),
            query,
            body,
        )
        .await
    }
    pub async fn get_repos_owner_repo_deployments_deployment_id<T, V>(
        &self,
        owner: String,
        repo: String,
        deployment_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoDeploymentsdeploymentIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoDeploymentsdeploymentId(owner, repo, deployment_id),
            query,
            body,
        )
        .await
    }
    pub async fn get_repos_owner_repo_deployments_deployment_id_statuses_status_id<T, V>(
        &self,
        owner: String,
        repo: String,
        deployment_id: String,
        status_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoDeploymentsdeploymentIdStatusesstatusIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoDeploymentsdeploymentIdStatusesstatusId(
                owner,
                repo,
                deployment_id,
                status_id,
            ),
            query,
            body,
        )
        .await
    }
    pub async fn get_repos_owner_repo_environments<T, V>(
        &self,
        owner: String,
        repo: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoEnvironmentsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoEnvironments(owner, repo),
            query,
            body,
        )
        .await
    }
    pub async fn get_repos_owner_repo_environments_environment_name<T, V>(
        &self,
        owner: String,
        repo: String,
        environment_name: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoEnvironmentsenvironmentNameResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoEnvironmentsenvironmentName(owner, repo, environment_name),
            query,
            body,
        )
        .await
    }
    pub async fn put_repos_owner_repo_environments_environment_name<T, V>(
        &self,
        owner: String,
        repo: String,
        environment_name: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PutReposownerrepoEnvironmentsenvironmentNameResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PutReposownerrepoEnvironmentsenvironmentName(owner, repo, environment_name),
            query,
            body,
        )
        .await
    }
    pub async fn get_repos_owner_repo_git_blobs_file_sha<T, V>(
        &self,
        owner: String,
        repo: String,
        file_sha: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoGitBlobsfileShaResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoGitBlobsfileSha(owner, repo, file_sha),
            query,
            body,
        )
        .await
    }
    pub async fn get_repos_owner_repo_git_commits_commit_sha<T, V>(
        &self,
        owner: String,
        repo: String,
        commit_sha: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoGitCommitscommitShaResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoGitCommitscommitSha(owner, repo, commit_sha),
            query,
            body,
        )
        .await
    }
    pub async fn get_repos_owner_repo_git_ref_aref<T, V>(
        &self,
        owner: String,
        repo: String,
        aref: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoGitRefrefResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoGitRefref(owner, repo, aref),
            query,
            body,
        )
        .await
    }
    pub async fn patch_repos_owner_repo_git_refs_aref<T, V>(
        &self,
        owner: String,
        repo: String,
        aref: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PatchReposownerrepoGitRefsrefResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchReposownerrepoGitRefsref(owner, repo, aref),
            query,
            body,
        )
        .await
    }
    pub async fn get_repos_owner_repo_git_tags_tag_sha<T, V>(
        &self,
        owner: String,
        repo: String,
        tag_sha: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoGitTagstagShaResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoGitTagstagSha(owner, repo, tag_sha),
            query,
            body,
        )
        .await
    }
    pub async fn get_repos_owner_repo_git_trees_tree_sha<T, V>(
        &self,
        owner: String,
        repo: String,
        tree_sha: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoGitTreestreeShaResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoGitTreestreeSha(owner, repo, tree_sha),
            query,
            body,
        )
        .await
    }
    pub async fn get_repos_owner_repo_hooks_hook_id<T, V>(
        &self,
        owner: String,
        repo: String,
        hook_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoHookshookIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoHookshookId(owner, repo, hook_id),
            query,
            body,
        )
        .await
    }
    pub async fn patch_repos_owner_repo_hooks_hook_id<T, V>(
        &self,
        owner: String,
        repo: String,
        hook_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PatchReposownerrepoHookshookIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchReposownerrepoHookshookId(owner, repo, hook_id),
            query,
            body,
        )
        .await
    }
    pub async fn get_repos_owner_repo_hooks_hook_id_config<T, V>(
        &self,
        owner: String,
        repo: String,
        hook_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoHookshookIdConfigResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoHookshookIdConfig(owner, repo, hook_id),
            query,
            body,
        )
        .await
    }
    pub async fn patch_repos_owner_repo_hooks_hook_id_config<T, V>(
        &self,
        owner: String,
        repo: String,
        hook_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PatchReposownerrepoHookshookIdConfigResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchReposownerrepoHookshookIdConfig(owner, repo, hook_id),
            query,
            body,
        )
        .await
    }
    pub async fn get_repos_owner_repo_hooks_hook_id_deliveries_delivery_id<T, V>(
        &self,
        owner: String,
        repo: String,
        hook_id: String,
        delivery_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoHookshookIdDeliveriesdeliveryIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoHookshookIdDeliveriesdeliveryId(
                owner,
                repo,
                hook_id,
                delivery_id,
            ),
            query,
            body,
        )
        .await
    }
    pub async fn get_repos_owner_repo_import<T, V>(
        &self,
        owner: String,
        repo: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoImportResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetReposownerrepoImport(owner, repo), query, body)
            .await
    }
    pub async fn patch_repos_owner_repo_import<T, V>(
        &self,
        owner: String,
        repo: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PatchReposownerrepoImportResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchReposownerrepoImport(owner, repo),
            query,
            body,
        )
        .await
    }
    pub async fn patch_repos_owner_repo_import_authors_author_id<T, V>(
        &self,
        owner: String,
        repo: String,
        author_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PatchReposownerrepoImportAuthorsauthorIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchReposownerrepoImportAuthorsauthorId(owner, repo, author_id),
            query,
            body,
        )
        .await
    }
    pub async fn patch_repos_owner_repo_import_lfs<T, V>(
        &self,
        owner: String,
        repo: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PatchReposownerrepoImportLfsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchReposownerrepoImportLfs(owner, repo),
            query,
            body,
        )
        .await
    }
    pub async fn get_repos_owner_repo_installation<T, V>(
        &self,
        owner: String,
        repo: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoInstallationResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoInstallation(owner, repo),
            query,
            body,
        )
        .await
    }
    pub async fn put_repos_owner_repo_interaction_limits<T, V>(
        &self,
        owner: String,
        repo: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PutReposownerrepoInteractionLimitsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PutReposownerrepoInteractionLimits(owner, repo),
            query,
            body,
        )
        .await
    }
    pub async fn patch_repos_owner_repo_invitations_invitation_id<T, V>(
        &self,
        owner: String,
        repo: String,
        invitation_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PatchReposownerrepoInvitationsinvitationIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchReposownerrepoInvitationsinvitationId(owner, repo, invitation_id),
            query,
            body,
        )
        .await
    }
    pub async fn get_repos_owner_repo_issues_comments_comment_id<T, V>(
        &self,
        owner: String,
        repo: String,
        comment_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoIssuesCommentscommentIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoIssuesCommentscommentId(owner, repo, comment_id),
            query,
            body,
        )
        .await
    }
    pub async fn patch_repos_owner_repo_issues_comments_comment_id<T, V>(
        &self,
        owner: String,
        repo: String,
        comment_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PatchReposownerrepoIssuesCommentscommentIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchReposownerrepoIssuesCommentscommentId(owner, repo, comment_id),
            query,
            body,
        )
        .await
    }
    pub async fn post_repos_owner_repo_issues_comments_comment_id_reactions<T, V>(
        &self,
        owner: String,
        repo: String,
        comment_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PostReposownerrepoIssuesCommentscommentIdReactionsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PostReposownerrepoIssuesCommentscommentIdReactions(owner, repo, comment_id),
            query,
            body,
        )
        .await
    }
    pub async fn get_repos_owner_repo_issues_events_event_id<T, V>(
        &self,
        owner: String,
        repo: String,
        event_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoIssuesEventseventIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoIssuesEventseventId(owner, repo, event_id),
            query,
            body,
        )
        .await
    }
    pub async fn get_repos_owner_repo_issues_issue_number<T, V>(
        &self,
        owner: String,
        repo: String,
        issue_number: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoIssuesissueNumberResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoIssuesissueNumber(owner, repo, issue_number),
            query,
            body,
        )
        .await
    }
    pub async fn patch_repos_owner_repo_issues_issue_number<T, V>(
        &self,
        owner: String,
        repo: String,
        issue_number: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PatchReposownerrepoIssuesissueNumberResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchReposownerrepoIssuesissueNumber(owner, repo, issue_number),
            query,
            body,
        )
        .await
    }
    pub async fn delete_repos_owner_repo_issues_issue_number_assignees<T, V>(
        &self,
        owner: String,
        repo: String,
        issue_number: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<DeleteReposownerrepoIssuesissueNumberAssigneesResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::DeleteReposownerrepoIssuesissueNumberAssignees(owner, repo, issue_number),
            query,
            body,
        )
        .await
    }
    pub async fn post_repos_owner_repo_issues_issue_number_reactions<T, V>(
        &self,
        owner: String,
        repo: String,
        issue_number: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PostReposownerrepoIssuesissueNumberReactionsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PostReposownerrepoIssuesissueNumberReactions(owner, repo, issue_number),
            query,
            body,
        )
        .await
    }
    pub async fn get_repos_owner_repo_keys_key_id<T, V>(
        &self,
        owner: String,
        repo: String,
        key_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoKeyskeyIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoKeyskeyId(owner, repo, key_id),
            query,
            body,
        )
        .await
    }
    pub async fn get_repos_owner_repo_labels_name<T, V>(
        &self,
        owner: String,
        repo: String,
        name: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoLabelsnameResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoLabelsname(owner, repo, name),
            query,
            body,
        )
        .await
    }
    pub async fn patch_repos_owner_repo_labels_name<T, V>(
        &self,
        owner: String,
        repo: String,
        name: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PatchReposownerrepoLabelsnameResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchReposownerrepoLabelsname(owner, repo, name),
            query,
            body,
        )
        .await
    }
    pub async fn get_repos_owner_repo_license<T, V>(
        &self,
        owner: String,
        repo: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoLicenseResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoLicense(owner, repo),
            query,
            body,
        )
        .await
    }
    pub async fn post_repos_owner_repo_merge_upstream<T, V>(
        &self,
        owner: String,
        repo: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PostReposownerrepoMergeUpstreamResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PostReposownerrepoMergeUpstream(owner, repo),
            query,
            body,
        )
        .await
    }
    pub async fn get_repos_owner_repo_milestones_milestone_number<T, V>(
        &self,
        owner: String,
        repo: String,
        milestone_number: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoMilestonesmilestoneNumberResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoMilestonesmilestoneNumber(owner, repo, milestone_number),
            query,
            body,
        )
        .await
    }
    pub async fn patch_repos_owner_repo_milestones_milestone_number<T, V>(
        &self,
        owner: String,
        repo: String,
        milestone_number: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PatchReposownerrepoMilestonesmilestoneNumberResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchReposownerrepoMilestonesmilestoneNumber(owner, repo, milestone_number),
            query,
            body,
        )
        .await
    }
    pub async fn get_repos_owner_repo_pages<T, V>(
        &self,
        owner: String,
        repo: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoPagesResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetReposownerrepoPages(owner, repo), query, body)
            .await
    }
    pub async fn get_repos_owner_repo_pages_builds_latest<T, V>(
        &self,
        owner: String,
        repo: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoPagesBuildsLatestResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoPagesBuildsLatest(owner, repo),
            query,
            body,
        )
        .await
    }
    pub async fn get_repos_owner_repo_pages_builds_build_id<T, V>(
        &self,
        owner: String,
        repo: String,
        build_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoPagesBuildsbuildIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoPagesBuildsbuildId(owner, repo, build_id),
            query,
            body,
        )
        .await
    }
    pub async fn get_repos_owner_repo_pages_health<T, V>(
        &self,
        owner: String,
        repo: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoPagesHealthResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoPagesHealth(owner, repo),
            query,
            body,
        )
        .await
    }
    pub async fn get_repos_owner_repo_pulls_comments_comment_id<T, V>(
        &self,
        owner: String,
        repo: String,
        comment_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoPullsCommentscommentIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoPullsCommentscommentId(owner, repo, comment_id),
            query,
            body,
        )
        .await
    }
    pub async fn patch_repos_owner_repo_pulls_comments_comment_id<T, V>(
        &self,
        owner: String,
        repo: String,
        comment_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PatchReposownerrepoPullsCommentscommentIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchReposownerrepoPullsCommentscommentId(owner, repo, comment_id),
            query,
            body,
        )
        .await
    }
    pub async fn post_repos_owner_repo_pulls_comments_comment_id_reactions<T, V>(
        &self,
        owner: String,
        repo: String,
        comment_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PostReposownerrepoPullsCommentscommentIdReactionsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PostReposownerrepoPullsCommentscommentIdReactions(owner, repo, comment_id),
            query,
            body,
        )
        .await
    }
    pub async fn get_repos_owner_repo_pulls_pull_number<T, V>(
        &self,
        owner: String,
        repo: String,
        pull_number: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoPullspullNumberResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoPullspullNumber(owner, repo, pull_number),
            query,
            body,
        )
        .await
    }
    pub async fn patch_repos_owner_repo_pulls_pull_number<T, V>(
        &self,
        owner: String,
        repo: String,
        pull_number: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PatchReposownerrepoPullspullNumberResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchReposownerrepoPullspullNumber(owner, repo, pull_number),
            query,
            body,
        )
        .await
    }
    pub async fn put_repos_owner_repo_pulls_pull_number_merge<T, V>(
        &self,
        owner: String,
        repo: String,
        pull_number: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PutReposownerrepoPullspullNumberMergeResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PutReposownerrepoPullspullNumberMerge(owner, repo, pull_number),
            query,
            body,
        )
        .await
    }
    pub async fn get_repos_owner_repo_pulls_pull_number_requested_reviewers<T, V>(
        &self,
        owner: String,
        repo: String,
        pull_number: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoPullspullNumberRequestedReviewersResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoPullspullNumberRequestedReviewers(owner, repo, pull_number),
            query,
            body,
        )
        .await
    }
    pub async fn delete_repos_owner_repo_pulls_pull_number_requested_reviewers<T, V>(
        &self,
        owner: String,
        repo: String,
        pull_number: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<DeleteReposownerrepoPullspullNumberRequestedReviewersResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::DeleteReposownerrepoPullspullNumberRequestedReviewers(
                owner,
                repo,
                pull_number,
            ),
            query,
            body,
        )
        .await
    }
    pub async fn post_repos_owner_repo_pulls_pull_number_reviews<T, V>(
        &self,
        owner: String,
        repo: String,
        pull_number: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PostReposownerrepoPullspullNumberReviewsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PostReposownerrepoPullspullNumberReviews(owner, repo, pull_number),
            query,
            body,
        )
        .await
    }
    pub async fn get_repos_owner_repo_pulls_pull_number_reviews_review_id<T, V>(
        &self,
        owner: String,
        repo: String,
        pull_number: String,
        review_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoPullspullNumberReviewsreviewIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoPullspullNumberReviewsreviewId(
                owner,
                repo,
                pull_number,
                review_id,
            ),
            query,
            body,
        )
        .await
    }
    pub async fn put_repos_owner_repo_pulls_pull_number_reviews_review_id<T, V>(
        &self,
        owner: String,
        repo: String,
        pull_number: String,
        review_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PutReposownerrepoPullspullNumberReviewsreviewIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PutReposownerrepoPullspullNumberReviewsreviewId(
                owner,
                repo,
                pull_number,
                review_id,
            ),
            query,
            body,
        )
        .await
    }
    pub async fn delete_repos_owner_repo_pulls_pull_number_reviews_review_id<T, V>(
        &self,
        owner: String,
        repo: String,
        pull_number: String,
        review_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<DeleteReposownerrepoPullspullNumberReviewsreviewIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::DeleteReposownerrepoPullspullNumberReviewsreviewId(
                owner,
                repo,
                pull_number,
                review_id,
            ),
            query,
            body,
        )
        .await
    }
    pub async fn put_repos_owner_repo_pulls_pull_number_reviews_review_id_dismissals<T, V>(
        &self,
        owner: String,
        repo: String,
        pull_number: String,
        review_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PutReposownerrepoPullspullNumberReviewsreviewIdDismissalsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PutReposownerrepoPullspullNumberReviewsreviewIdDismissals(
                owner,
                repo,
                pull_number,
                review_id,
            ),
            query,
            body,
        )
        .await
    }
    pub async fn post_repos_owner_repo_pulls_pull_number_reviews_review_id_events<T, V>(
        &self,
        owner: String,
        repo: String,
        pull_number: String,
        review_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PostReposownerrepoPullspullNumberReviewsreviewIdEventsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PostReposownerrepoPullspullNumberReviewsreviewIdEvents(
                owner,
                repo,
                pull_number,
                review_id,
            ),
            query,
            body,
        )
        .await
    }
    pub async fn get_repos_owner_repo_readme<T, V>(
        &self,
        owner: String,
        repo: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoReadmeResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetReposownerrepoReadme(owner, repo), query, body)
            .await
    }
    pub async fn get_repos_owner_repo_readme_dir<T, V>(
        &self,
        owner: String,
        repo: String,
        dir: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoReadmedirResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoReadmedir(owner, repo, dir),
            query,
            body,
        )
        .await
    }
    pub async fn get_repos_owner_repo_releases_assets_asset_id<T, V>(
        &self,
        owner: String,
        repo: String,
        asset_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoReleasesAssetsassetIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoReleasesAssetsassetId(owner, repo, asset_id),
            query,
            body,
        )
        .await
    }
    pub async fn patch_repos_owner_repo_releases_assets_asset_id<T, V>(
        &self,
        owner: String,
        repo: String,
        asset_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PatchReposownerrepoReleasesAssetsassetIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchReposownerrepoReleasesAssetsassetId(owner, repo, asset_id),
            query,
            body,
        )
        .await
    }
    pub async fn post_repos_owner_repo_releases_generate_notes<T, V>(
        &self,
        owner: String,
        repo: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PostReposownerrepoReleasesGenerateNotesResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PostReposownerrepoReleasesGenerateNotes(owner, repo),
            query,
            body,
        )
        .await
    }
    pub async fn get_repos_owner_repo_releases_latest<T, V>(
        &self,
        owner: String,
        repo: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoReleasesLatestResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoReleasesLatest(owner, repo),
            query,
            body,
        )
        .await
    }
    pub async fn get_repos_owner_repo_releases_tags_tag<T, V>(
        &self,
        owner: String,
        repo: String,
        tag: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoReleasesTagstagResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoReleasesTagstag(owner, repo, tag),
            query,
            body,
        )
        .await
    }
    pub async fn get_repos_owner_repo_releases_release_id<T, V>(
        &self,
        owner: String,
        repo: String,
        release_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoReleasesreleaseIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoReleasesreleaseId(owner, repo, release_id),
            query,
            body,
        )
        .await
    }
    pub async fn patch_repos_owner_repo_releases_release_id<T, V>(
        &self,
        owner: String,
        repo: String,
        release_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PatchReposownerrepoReleasesreleaseIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchReposownerrepoReleasesreleaseId(owner, repo, release_id),
            query,
            body,
        )
        .await
    }
    pub async fn post_repos_owner_repo_releases_release_id_reactions<T, V>(
        &self,
        owner: String,
        repo: String,
        release_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PostReposownerrepoReleasesreleaseIdReactionsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PostReposownerrepoReleasesreleaseIdReactions(owner, repo, release_id),
            query,
            body,
        )
        .await
    }
    pub async fn get_repos_owner_repo_secret_scanning_alerts_alert_number<T, V>(
        &self,
        owner: String,
        repo: String,
        alert_number: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoSecretScanningAlertsalertNumberResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoSecretScanningAlertsalertNumber(owner, repo, alert_number),
            query,
            body,
        )
        .await
    }
    pub async fn patch_repos_owner_repo_secret_scanning_alerts_alert_number<T, V>(
        &self,
        owner: String,
        repo: String,
        alert_number: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PatchReposownerrepoSecretScanningAlertsalertNumberResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchReposownerrepoSecretScanningAlertsalertNumber(
                owner,
                repo,
                alert_number,
            ),
            query,
            body,
        )
        .await
    }
    pub async fn get_repos_owner_repo_stats_participation<T, V>(
        &self,
        owner: String,
        repo: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoStatsParticipationResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoStatsParticipation(owner, repo),
            query,
            body,
        )
        .await
    }
    pub async fn get_repos_owner_repo_subscription<T, V>(
        &self,
        owner: String,
        repo: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoSubscriptionResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoSubscription(owner, repo),
            query,
            body,
        )
        .await
    }
    pub async fn put_repos_owner_repo_subscription<T, V>(
        &self,
        owner: String,
        repo: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PutReposownerrepoSubscriptionResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PutReposownerrepoSubscription(owner, repo),
            query,
            body,
        )
        .await
    }
    pub async fn get_repos_owner_repo_topics<T, V>(
        &self,
        owner: String,
        repo: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoTopicsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetReposownerrepoTopics(owner, repo), query, body)
            .await
    }
    pub async fn put_repos_owner_repo_topics<T, V>(
        &self,
        owner: String,
        repo: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PutReposownerrepoTopicsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::PutReposownerrepoTopics(owner, repo), query, body)
            .await
    }
    pub async fn get_repos_owner_repo_traffic_clones<T, V>(
        &self,
        owner: String,
        repo: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoTrafficClonesResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoTrafficClones(owner, repo),
            query,
            body,
        )
        .await
    }
    pub async fn get_repos_owner_repo_traffic_views<T, V>(
        &self,
        owner: String,
        repo: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetReposownerrepoTrafficViewsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoTrafficViews(owner, repo),
            query,
            body,
        )
        .await
    }
    pub async fn get_repositories_repository_id_environments_environment_name_secrets<T, V>(
        &self,
        repository_id: String,
        environment_name: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetRepositoriesrepositoryIdEnvironmentsenvironmentNameSecretsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetRepositoriesrepositoryIdEnvironmentsenvironmentNameSecrets(
                repository_id,
                environment_name,
            ),
            query,
            body,
        )
        .await
    }
    pub async fn get_repositories_repository_id_environments_environment_name_secrets_public_key<
        T,
        V,
    >(
        &self,
        repository_id: String,
        environment_name: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetRepositoriesrepositoryIdEnvironmentsenvironmentNameSecretsPublicKeyResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetRepositoriesrepositoryIdEnvironmentsenvironmentNameSecretsPublicKey(
                repository_id,
                environment_name,
            ),
            query,
            body,
        )
        .await
    }
    pub async fn get_repositories_repository_id_environments_environment_name_secrets_secret_name<
        T,
        V,
    >(
        &self,
        repository_id: String,
        environment_name: String,
        secret_name: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<
        GetRepositoriesrepositoryIdEnvironmentsenvironmentNameSecretssecretNameResponse,
        Error,
    >
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetRepositoriesrepositoryIdEnvironmentsenvironmentNameSecretssecretName(
                repository_id,
                environment_name,
                secret_name,
            ),
            query,
            body,
        )
        .await
    }
    pub async fn get_scim_v2_enterprises_enterprise_Groups<T, V>(
        &self,
        enterprise: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetScimV2EnterprisesenterpriseGroupsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetScimV2EnterprisesenterpriseGroups(enterprise),
            query,
            body,
        )
        .await
    }
    pub async fn get_scim_v2_enterprises_enterprise_Groups_scim_group_id<T, V>(
        &self,
        enterprise: String,
        scim_group_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetScimV2EnterprisesenterpriseGroupsscimGroupIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetScimV2EnterprisesenterpriseGroupsscimGroupId(enterprise, scim_group_id),
            query,
            body,
        )
        .await
    }
    pub async fn put_scim_v2_enterprises_enterprise_Groups_scim_group_id<T, V>(
        &self,
        enterprise: String,
        scim_group_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PutScimV2EnterprisesenterpriseGroupsscimGroupIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PutScimV2EnterprisesenterpriseGroupsscimGroupId(enterprise, scim_group_id),
            query,
            body,
        )
        .await
    }
    pub async fn patch_scim_v2_enterprises_enterprise_Groups_scim_group_id<T, V>(
        &self,
        enterprise: String,
        scim_group_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PatchScimV2EnterprisesenterpriseGroupsscimGroupIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchScimV2EnterprisesenterpriseGroupsscimGroupId(enterprise, scim_group_id),
            query,
            body,
        )
        .await
    }
    pub async fn get_scim_v2_enterprises_enterprise_Users<T, V>(
        &self,
        enterprise: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetScimV2EnterprisesenterpriseUsersResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetScimV2EnterprisesenterpriseUsers(enterprise),
            query,
            body,
        )
        .await
    }
    pub async fn get_scim_v2_enterprises_enterprise_Users_scim_user_id<T, V>(
        &self,
        enterprise: String,
        scim_user_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetScimV2EnterprisesenterpriseUsersscimUserIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetScimV2EnterprisesenterpriseUsersscimUserId(enterprise, scim_user_id),
            query,
            body,
        )
        .await
    }
    pub async fn put_scim_v2_enterprises_enterprise_Users_scim_user_id<T, V>(
        &self,
        enterprise: String,
        scim_user_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PutScimV2EnterprisesenterpriseUsersscimUserIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PutScimV2EnterprisesenterpriseUsersscimUserId(enterprise, scim_user_id),
            query,
            body,
        )
        .await
    }
    pub async fn patch_scim_v2_enterprises_enterprise_Users_scim_user_id<T, V>(
        &self,
        enterprise: String,
        scim_user_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PatchScimV2EnterprisesenterpriseUsersscimUserIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchScimV2EnterprisesenterpriseUsersscimUserId(enterprise, scim_user_id),
            query,
            body,
        )
        .await
    }
    pub async fn get_search_code<T, V>(
        &self,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetSearchCodeResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetSearchCode(), query, body).await
    }
    pub async fn get_search_commits<T, V>(
        &self,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetSearchCommitsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetSearchCommits(), query, body).await
    }
    pub async fn get_search_issues<T, V>(
        &self,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetSearchIssuesResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetSearchIssues(), query, body).await
    }
    pub async fn get_search_labels<T, V>(
        &self,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetSearchLabelsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetSearchLabels(), query, body).await
    }
    pub async fn get_search_repositories<T, V>(
        &self,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetSearchRepositoriesResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetSearchRepositories(), query, body)
            .await
    }
    pub async fn get_search_topics<T, V>(
        &self,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetSearchTopicsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetSearchTopics(), query, body).await
    }
    pub async fn get_search_users<T, V>(
        &self,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetSearchUsersResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetSearchUsers(), query, body).await
    }
    pub async fn get_teams_team_id<T, V>(
        &self,
        team_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetTeamsteamIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetTeamsteamId(team_id), query, body)
            .await
    }
    pub async fn patch_teams_team_id<T, V>(
        &self,
        team_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PatchTeamsteamIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::PatchTeamsteamId(team_id), query, body)
            .await
    }
    pub async fn get_teams_team_id_discussions_discussion_number<T, V>(
        &self,
        team_id: String,
        discussion_number: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetTeamsteamIdDiscussionsdiscussionNumberResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetTeamsteamIdDiscussionsdiscussionNumber(team_id, discussion_number),
            query,
            body,
        )
        .await
    }
    pub async fn patch_teams_team_id_discussions_discussion_number<T, V>(
        &self,
        team_id: String,
        discussion_number: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PatchTeamsteamIdDiscussionsdiscussionNumberResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchTeamsteamIdDiscussionsdiscussionNumber(team_id, discussion_number),
            query,
            body,
        )
        .await
    }
    pub async fn get_teams_team_id_discussions_discussion_number_comments_comment_number<T, V>(
        &self,
        team_id: String,
        discussion_number: String,
        comment_number: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetTeamsteamIdDiscussionsdiscussionNumberCommentscommentNumberResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetTeamsteamIdDiscussionsdiscussionNumberCommentscommentNumber(
                team_id,
                discussion_number,
                comment_number,
            ),
            query,
            body,
        )
        .await
    }
    pub async fn patch_teams_team_id_discussions_discussion_number_comments_comment_number<T, V>(
        &self,
        team_id: String,
        discussion_number: String,
        comment_number: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PatchTeamsteamIdDiscussionsdiscussionNumberCommentscommentNumberResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchTeamsteamIdDiscussionsdiscussionNumberCommentscommentNumber(
                team_id,
                discussion_number,
                comment_number,
            ),
            query,
            body,
        )
        .await
    }
    pub async fn get_teams_team_id_memberships_username<T, V>(
        &self,
        team_id: String,
        username: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetTeamsteamIdMembershipsusernameResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetTeamsteamIdMembershipsusername(team_id, username),
            query,
            body,
        )
        .await
    }
    pub async fn put_teams_team_id_memberships_username<T, V>(
        &self,
        team_id: String,
        username: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PutTeamsteamIdMembershipsusernameResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PutTeamsteamIdMembershipsusername(team_id, username),
            query,
            body,
        )
        .await
    }
    pub async fn get_teams_team_id_projects_project_id<T, V>(
        &self,
        team_id: String,
        project_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetTeamsteamIdProjectsprojectIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetTeamsteamIdProjectsprojectId(team_id, project_id),
            query,
            body,
        )
        .await
    }
    pub async fn get_teams_team_id_repos_owner_repo<T, V>(
        &self,
        team_id: String,
        owner: String,
        repo: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetTeamsteamIdReposownerrepoResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetTeamsteamIdReposownerrepo(team_id, owner, repo),
            query,
            body,
        )
        .await
    }
    pub async fn get_teams_team_id_team_sync_group_mappings<T, V>(
        &self,
        team_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetTeamsteamIdTeamSyncGroupMappingsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetTeamsteamIdTeamSyncGroupMappings(team_id),
            query,
            body,
        )
        .await
    }
    pub async fn patch_teams_team_id_team_sync_group_mappings<T, V>(
        &self,
        team_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PatchTeamsteamIdTeamSyncGroupMappingsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchTeamsteamIdTeamSyncGroupMappings(team_id),
            query,
            body,
        )
        .await
    }
    pub async fn patch_user<T, V>(
        &self,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PatchUserResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::PatchUser(), query, body).await
    }
    pub async fn get_user_codespaces<T, V>(
        &self,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetUserCodespacesResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetUserCodespaces(), query, body).await
    }
    pub async fn get_user_codespaces_secrets<T, V>(
        &self,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetUserCodespacesSecretsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetUserCodespacesSecrets(), query, body)
            .await
    }
    pub async fn get_user_codespaces_secrets_public_key<T, V>(
        &self,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetUserCodespacesSecretsPublicKeyResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetUserCodespacesSecretsPublicKey(), query, body)
            .await
    }
    pub async fn get_user_codespaces_secrets_secret_name<T, V>(
        &self,
        secret_name: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetUserCodespacesSecretssecretNameResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetUserCodespacesSecretssecretName(secret_name),
            query,
            body,
        )
        .await
    }
    pub async fn get_user_codespaces_secrets_secret_name_repositories<T, V>(
        &self,
        secret_name: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetUserCodespacesSecretssecretNameRepositoriesResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetUserCodespacesSecretssecretNameRepositories(secret_name),
            query,
            body,
        )
        .await
    }
    pub async fn get_user_codespaces_codespace_name<T, V>(
        &self,
        codespace_name: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetUserCodespacescodespaceNameResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetUserCodespacescodespaceName(codespace_name),
            query,
            body,
        )
        .await
    }
    pub async fn patch_user_codespaces_codespace_name<T, V>(
        &self,
        codespace_name: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PatchUserCodespacescodespaceNameResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchUserCodespacescodespaceName(codespace_name),
            query,
            body,
        )
        .await
    }
    pub async fn get_user_codespaces_codespace_name_machines<T, V>(
        &self,
        codespace_name: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetUserCodespacescodespaceNameMachinesResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetUserCodespacescodespaceNameMachines(codespace_name),
            query,
            body,
        )
        .await
    }
    pub async fn post_user_codespaces_codespace_name_start<T, V>(
        &self,
        codespace_name: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PostUserCodespacescodespaceNameStartResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PostUserCodespacescodespaceNameStart(codespace_name),
            query,
            body,
        )
        .await
    }
    pub async fn post_user_codespaces_codespace_name_stop<T, V>(
        &self,
        codespace_name: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PostUserCodespacescodespaceNameStopResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PostUserCodespacescodespaceNameStop(codespace_name),
            query,
            body,
        )
        .await
    }
    pub async fn get_user_gpg_keys_gpg_key_id<T, V>(
        &self,
        gpg_key_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetUserGpgKeysgpgKeyIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetUserGpgKeysgpgKeyId(gpg_key_id), query, body)
            .await
    }
    pub async fn get_user_installations<T, V>(
        &self,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetUserInstallationsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetUserInstallations(), query, body)
            .await
    }
    pub async fn get_user_installations_installation_id_repositories<T, V>(
        &self,
        installation_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetUserInstallationsinstallationIdRepositoriesResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetUserInstallationsinstallationIdRepositories(installation_id),
            query,
            body,
        )
        .await
    }
    pub async fn put_user_interaction_limits<T, V>(
        &self,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PutUserInteractionLimitsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::PutUserInteractionLimits(), query, body)
            .await
    }
    pub async fn get_user_keys_key_id<T, V>(
        &self,
        key_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetUserKeyskeyIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetUserKeyskeyId(key_id), query, body)
            .await
    }
    pub async fn get_user_memberships_orgs_org<T, V>(
        &self,
        org: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetUserMembershipsOrgsorgResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetUserMembershipsOrgsorg(org), query, body)
            .await
    }
    pub async fn patch_user_memberships_orgs_org<T, V>(
        &self,
        org: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<PatchUserMembershipsOrgsorgResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::PatchUserMembershipsOrgsorg(org), query, body)
            .await
    }
    pub async fn get_user_migrations_migration_id<T, V>(
        &self,
        migration_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetUserMigrationsmigrationIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetUserMigrationsmigrationId(migration_id),
            query,
            body,
        )
        .await
    }
    pub async fn get_user_packages_package_type_package_name<T, V>(
        &self,
        package_type: String,
        package_name: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetUserPackagespackageTypepackageNameResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetUserPackagespackageTypepackageName(package_type, package_name),
            query,
            body,
        )
        .await
    }
    pub async fn get_user_packages_package_type_package_name_versions_package_version_id<T, V>(
        &self,
        package_type: String,
        package_name: String,
        package_version_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetUserPackagespackageTypepackageNameVersionspackageVersionIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetUserPackagespackageTypepackageNameVersionspackageVersionId(
                package_type,
                package_name,
                package_version_id,
            ),
            query,
            body,
        )
        .await
    }
    pub async fn get_users_username_hovercard<T, V>(
        &self,
        username: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetUsersusernameHovercardResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetUsersusernameHovercard(username), query, body)
            .await
    }
    pub async fn get_users_username_installation<T, V>(
        &self,
        username: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetUsersusernameInstallationResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetUsersusernameInstallation(username),
            query,
            body,
        )
        .await
    }
    pub async fn get_users_username_packages_package_type_package_name<T, V>(
        &self,
        username: String,
        package_type: String,
        package_name: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetUsersusernamePackagespackageTypepackageNameResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetUsersusernamePackagespackageTypepackageName(
                username,
                package_type,
                package_name,
            ),
            query,
            body,
        )
        .await
    }
    pub async fn get_users_username_packages_package_type_package_name_versions_package_version_id<
        T,
        V,
    >(
        &self,
        username: String,
        package_type: String,
        package_name: String,
        package_version_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetUsersusernamePackagespackageTypepackageNameVersionspackageVersionIdResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetUsersusernamePackagespackageTypepackageNameVersionspackageVersionId(
                username,
                package_type,
                package_name,
                package_version_id,
            ),
            query,
            body,
        )
        .await
    }
    pub async fn get_users_username_settings_billing_actions<T, V>(
        &self,
        username: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetUsersusernameSettingsBillingActionsResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetUsersusernameSettingsBillingActions(username),
            query,
            body,
        )
        .await
    }
    pub async fn get_users_username_settings_billing_packages<T, V>(
        &self,
        username: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetUsersusernameSettingsBillingPackagesResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetUsersusernameSettingsBillingPackages(username),
            query,
            body,
        )
        .await
    }
    pub async fn get_users_username_settings_billing_shared_storage<T, V>(
        &self,
        username: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<GetUsersusernameSettingsBillingSharedStorageResponse, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetUsersusernameSettingsBillingSharedStorage(username),
            query,
            body,
        )
        .await
    }
}
